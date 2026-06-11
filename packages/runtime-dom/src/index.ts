import type {
  FrameIrAttribute,
  FrameIrAttributeValue,
  FrameIrBinding,
  FrameIrComponent,
  FrameIrComponentArgumentValue,
  FrameIrCondition,
  FrameIrDocument,
  FrameIrElement,
  FrameIrEvent,
  FrameIrList,
  FrameIrNode,
  FrameIrStateDefault,
  FrameIrStyleBinding,
  FrameIrTextValue
} from './ir.js';

export type FrameRuntimeValue =
  | string
  | number
  | boolean
  | null
  | FrameRuntimeValue[]
  | { [key: string]: FrameRuntimeValue };

export type FrameHandlerContext = {
  event: Event;
  state: FrameStateController;
  props: Readonly<Record<string, FrameRuntimeValue>>;
};

export type FrameHandler = (context: FrameHandlerContext) => void | Promise<void>;
export type FrameHandlerMap = Record<string, FrameHandler>;

export type FrameRuntimeCounters = {
  mounts: number;
  unmounts: number;
  mountedComponents: number;
  patchedTexts: number;
  patchedAttributes: number;
  patchedProperties: number;
  patchedConditions: number;
  patchedStyles: number;
  patchedLists: number;
  queuedPatches: number;
  flushedPatches: number;
  activeListeners: number;
  activeSubscriptions: number;
  disposedNodes: number;
  listMoves: number;
  listReuses: number;
  listCreates: number;
  listRemoves: number;
  runtimeErrors: number;
};

export type MountOptions = {
  component: string;
  target: Element;
  props?: Record<string, FrameRuntimeValue>;
  handlers?: FrameHandlerMap;
  debug?: boolean;
};

export type MountedFrameApp = {
  component: string;
  target: Element;
  state: FrameStateController;
  flush(): void;
  dispose(): void;
  getDebugStats(): FrameRuntimeCounters;
  resetDebugStats(): void;
};

export class FrameDomError extends Error {
  component?: string;
  source?: { start: number; end: number };

  constructor(message: string, options: { component?: string; source?: { start: number; end: number }; cause?: unknown } = {}) {
    const suffix = [
      options.component ? `component ${options.component}` : null,
      options.source ? `source ${options.source.start}..${options.source.end}` : null
    ].filter(Boolean).join(', ');
    super(suffix ? `${message} (${suffix})` : message, { cause: options.cause });
    this.name = 'FrameDomError';
    this.component = options.component;
    this.source = options.source;
  }
}

type ScheduledPatch = {
  id: number;
  order: number;
  label: string;
  component: string;
  source?: { start: number; end: number };
  run: () => void;
};

class FrameScheduler {
  #queue = new Map<number, ScheduledPatch>();
  #scheduled = false;
  #flushing = false;
  #nextId = 1;

  constructor(
    private readonly counters: FrameRuntimeCounters,
    private readonly debug: boolean
  ) {}

  createPatch(
    label: string,
    component: string,
    source: { start: number; end: number } | undefined,
    run: () => void
  ): ScheduledPatch {
    const id = this.#nextId++;
    return { id, order: id, label, component, source, run };
  }

  enqueue(patch: ScheduledPatch): void {
    if (!this.#queue.has(patch.id)) {
      this.#queue.set(patch.id, patch);
      this.counters.queuedPatches += 1;
      if (this.debug) {
        console.debug(`[Frame Runtime] queued ${patch.label} (${patch.component})`);
      }
    }
    this.#schedule();
  }

  flush(): void {
    if (this.#flushing) {
      return;
    }
    this.#scheduled = false;
    this.#flushing = true;
    let cycles = 0;
    try {
      while (this.#queue.size > 0) {
        cycles += 1;
        if (cycles > 25) {
          throw new FrameDomError('Recursive update loop detected while flushing scheduled patches.');
        }
        const patches = [...this.#queue.values()].sort((left, right) => left.order - right.order);
        this.#queue.clear();
        for (const patch of patches) {
          try {
            if (this.debug) {
              console.debug(`[Frame Runtime] flushing ${patch.label} (${patch.component})`);
            }
            patch.run();
            this.counters.flushedPatches += 1;
          } catch (error) {
            this.counters.runtimeErrors += 1;
            throw runtimeError(`Patch failed: ${patch.label}. ${errorMessage(error)}`, patch.component, patch.source, error);
          }
        }
      }
    } finally {
      this.#flushing = false;
      if (this.#queue.size > 0) {
        this.#schedule();
      }
    }
  }

  get queuedCount(): number {
    return this.#queue.size;
  }

  clear(): void {
    this.#queue.clear();
    this.#scheduled = false;
  }

  #schedule(): void {
    if (this.#scheduled || this.#flushing) {
      return;
    }
    this.#scheduled = true;
    queueMicrotask(() => this.flush());
  }
}

export class FrameStateController {
  #values: Record<string, FrameRuntimeValue>;
  #disposed = false;
  #subscribers = new Map<string, Set<() => void>>();
  #notifyDepth = 0;

  constructor(values: Record<string, FrameRuntimeValue>, private readonly componentName?: string) {
    this.#values = { ...values };
  }

  get(name: string): FrameRuntimeValue {
    if (!Object.hasOwn(this.#values, rootName(name))) {
      throw new FrameDomError(`Unknown state value \`${name}\`.`, { component: this.componentName });
    }
    return readPath(this.#values, name);
  }

  set(name: string, value: FrameRuntimeValue): void {
    if (this.#disposed) {
      return;
    }
    if (!Object.hasOwn(this.#values, rootName(name))) {
      throw new FrameDomError(`Unknown state value \`${name}\`.`, { component: this.componentName });
    }
    if (Object.is(this.get(name), value)) {
      return;
    }
    writePath(this.#values, name, value);
    this.#notify(rootName(name));
  }

  snapshot(): Record<string, FrameRuntimeValue> {
    return { ...this.#values };
  }

  subscribe(subscriber: () => void): () => void {
    return this.subscribeTo(Object.keys(this.#values), subscriber);
  }

  subscribeTo(dependencies: Iterable<string>, subscriber: () => void): () => void {
    if (this.#disposed) {
      return () => {};
    }
    const keys = [...new Set([...dependencies].map(rootName))];
    for (const key of keys) {
      let subscribers = this.#subscribers.get(key);
      if (!subscribers) {
        subscribers = new Set();
        this.#subscribers.set(key, subscribers);
      }
      subscribers.add(subscriber);
    }
    return () => {
      for (const key of keys) {
        this.#subscribers.get(key)?.delete(subscriber);
      }
    };
  }

  dispose(): void {
    this.#disposed = true;
    this.#subscribers.clear();
  }

  #notify(name: string): void {
    this.#notifyDepth += 1;
    try {
      if (this.#notifyDepth > 25) {
        throw new FrameDomError('Recursive state update loop detected.', { component: this.componentName });
      }
      for (const subscriber of [...(this.#subscribers.get(name) ?? [])]) {
        subscriber();
      }
    } finally {
      this.#notifyDepth -= 1;
    }
  }
}

type RenderContext = {
  document: Document;
  components: Map<string, FrameIrComponent>;
  component: FrameIrComponent;
  state: FrameStateController;
  props: Readonly<Record<string, FrameRuntimeValue>>;
  handlers: FrameHandlerMap;
  cleanup: Array<() => void>;
  scope: Record<string, FrameRuntimeValue>;
  localPatches?: Array<() => void>;
  debug: boolean;
  counters: FrameRuntimeCounters;
  scheduler: FrameScheduler;
};

type RenderedBlock = {
  nodes: Node[];
  cleanup: Array<() => void>;
};

const ELEMENT_TAGS: Record<string, string> = {
  a: 'a',
  action: 'button',
  area: 'div',
  article: 'article',
  audio: 'audio',
  avatar: 'img',
  badge: 'span',
  button: 'button',
  canvas: 'canvas',
  card: 'div',
  caption: 'caption',
  choice: 'select',
  col: 'col',
  colgroup: 'colgroup',
  composer: 'form',
  data: 'table',
  dd: 'dd',
  details: 'details',
  dialog: 'dialog',
  div: 'div',
  dock: 'div',
  dl: 'dl',
  dt: 'dt',
  editor: 'textarea',
  empty: 'div',
  feed: 'div',
  field: 'div',
  fieldset: 'fieldset',
  footer: 'footer',
  form: 'form',
  grid: 'div',
  h1: 'h1',
  h2: 'h2',
  h3: 'h3',
  h4: 'h4',
  h5: 'h5',
  h6: 'h6',
  header: 'header',
  icon: 'span',
  image: 'img',
  img: 'img',
  input: 'input',
  item: 'li',
  label: 'label',
  legend: 'legend',
  link: 'a',
  li: 'li',
  list: 'ul',
  main: 'main',
  media: 'video',
  menu: 'nav',
  meter: 'meter',
  nav: 'nav',
  ol: 'ol',
  optgroup: 'optgroup',
  option: 'option',
  output: 'output',
  overlay: 'div',
  p: 'p',
  panel: 'section',
  path: 'path',
  picture: 'picture',
  popover: 'div',
  progress: 'progress',
  row: 'div',
  scroll: 'div',
  screen: 'div',
  section: 'section',
  select: 'select',
  source: 'source',
  span: 'span',
  stack: 'div',
  summary: 'summary',
  svg: 'svg',
  table: 'table',
  tabs: 'div',
  tbody: 'tbody',
  td: 'td',
  textarea: 'textarea',
  text: 'span',
  tfoot: 'tfoot',
  th: 'th',
  thead: 'thead',
  title: 'h2',
  toggle: 'input',
  toolbar: 'div',
  track: 'track',
  tr: 'tr',
  ul: 'ul',
  video: 'video'
};

const SVG_TAGS = new Set(['svg', 'path']);
const SVG_NAMESPACE = 'http://www.w3.org/2000/svg';
const USER_CLASSES = new WeakMap<Element, Set<string>>();

export function defineFrameIrDocument<const T extends FrameIrDocument>(ir: T): T {
  return ir;
}

export function mount(ir: FrameIrDocument, options: MountOptions): MountedFrameApp {
  const components = new Map(ir.components.map((component) => [component.name, component]));
  const component = components.get(options.component);
  if (!component) {
    throw new FrameDomError(`Unknown component \`${options.component}\`.`);
  }

  validateHandlers(component, options.handlers ?? {}, options.debug ?? false);
  validateProps(component, options.props ?? {});

  const cleanup: Array<() => void> = [];
  const counters = emptyCounters();
  counters.mounts += 1;
  counters.mountedComponents += 1;
  const scheduler = new FrameScheduler(counters, options.debug ?? false);
  const context: RenderContext = {
    document: options.target.ownerDocument,
    components,
    component,
    state: new FrameStateController(initializeState(component), component.name),
    props: initializeProps(component, options.props ?? {}),
    handlers: options.handlers ?? {},
    cleanup,
    scope: {},
    debug: options.debug ?? false,
    counters,
    scheduler
  };

  for (const node of component.nodes) {
    appendBlock(options.target, renderNode(node, context));
  }

  return {
    component: component.name,
    target: options.target,
    state: context.state,
    flush() {
      scheduler.flush();
    },
    dispose() {
      cleanup.splice(0).forEach((dispose) => dispose());
      scheduler.clear();
      context.state.dispose();
      counters.disposedNodes += options.target.childNodes.length;
      options.target.replaceChildren();
      counters.unmounts += 1;
      counters.mountedComponents = 0;
    },
    getDebugStats() {
      return { ...counters, queuedPatches: scheduler.queuedCount };
    },
    resetDebugStats() {
      const activeListeners = counters.activeListeners;
      const activeSubscriptions = counters.activeSubscriptions;
      const mountedComponents = counters.mountedComponents;
      Object.assign(counters, emptyCounters(), {
        activeListeners,
        activeSubscriptions,
        mountedComponents
      });
    }
  };
}

function initializeProps(
  component: FrameIrComponent,
  values: Record<string, FrameRuntimeValue>
): Readonly<Record<string, FrameRuntimeValue>> {
  const props: Record<string, FrameRuntimeValue> = {};
  for (const descriptor of component.props) {
    props[descriptor.name] = values[descriptor.name] ?? null;
  }
  return props;
}

function initializeState(component: FrameIrComponent): Record<string, FrameRuntimeValue> {
  const state: Record<string, FrameRuntimeValue> = {};
  for (const descriptor of component.state) {
    state[descriptor.name] = defaultValue(descriptor.default);
  }
  return state;
}

function defaultValue(value: FrameIrStateDefault): FrameRuntimeValue {
  if (value === 'List') {
    return [];
  }
  if ('Text' in value) {
    return value.Text;
  }
  if ('Bool' in value) {
    return value.Bool;
  }
  if ('Number' in value) {
    return Number(value.Number);
  }
  return null;
}

function renderNode(node: FrameIrNode, context: RenderContext): RenderedBlock {
  if ('Element' in node) {
    return renderElement(node.Element, context);
  }
  if ('Text' in node) {
    return renderText(node.Text.value, context);
  }
  if ('Component' in node) {
    return renderComponentInvocation(node.Component.name, node.Component.arguments, context);
  }
  return renderList(node.List, context);
}

function renderText(value: FrameIrTextValue, context: RenderContext): RenderedBlock {
  const span = context.document.createElement('span');
  span.className = 'fr-FrameText';
  const node = context.document.createTextNode(String(readText(value, context)));
  span.appendChild(node);
  if ('DataRef' in value) {
    const patch = () => {
      node.nodeValue = String(readText(value, context));
      count(context, 'patchedTexts', 'patched text node');
    };
    if (isStateDependency(value.DataRef, context)) {
      subscribePatch(context, [value.DataRef], 'text node', undefined, patch);
    } else {
      registerLocalPatch(context, patch);
    }
  }
  return block([span]);
}

function renderElement(element: FrameIrElement, context: RenderContext): RenderedBlock {
  const renderKind = element.render_kind ?? element.kind;
  const tag = ELEMENT_TAGS[renderKind];
  if (!tag) {
    throw runtimeError(`Unsupported element kind \`${renderKind}\`.`, context.component.name, element.source);
  }

  const dom = SVG_TAGS.has(tag)
    ? context.document.createElementNS(SVG_NAMESPACE, tag)
    : context.document.createElement(tag);
  const anchor = context.document.createComment(`frame:${element.name}`);
  applySemanticDefaults(dom, element);

  applyStyles(dom, element.style, element.conditions, context);
  applyAttributes(dom, element.attributes, context);
  applyConditionalProperties(dom, element.conditions, context);
  applyShow(dom, element.conditions, context);
  attachEvents(dom, element.events, context);
  appendSemanticContent(dom, element, context);

  for (const child of element.children) {
    appendBlock(dom, renderNode(child, context));
  }
  applyBindings(dom, element.bindings, context);

  return block([anchor, dom]);
}

function appendSemanticContent(dom: Element, element: FrameIrElement, context: RenderContext): void {
  const semanticKind = element.semantic_kind ?? element.kind;
  if (!['title', 'text', 'label', 'badge', 'icon', 'action', 'link', 'menu', 'composer'].includes(semanticKind)) {
    return;
  }
  const value = element.attributes.find((attribute) => attribute.name === 'value');
  if (!value) {
    return;
  }
  appendBlock(dom, renderText(value.value, context));
}

function applySemanticDefaults(dom: Element, element: FrameIrElement): void {
  const semanticKind = element.semantic_kind ?? element.kind;
  if (semanticKind === 'action' && dom.tagName.toLowerCase() === 'button' && !dom.hasAttribute('type')) {
    dom.setAttribute('type', 'button');
  }
  if (semanticKind === 'toggle' && dom.tagName.toLowerCase() === 'input') {
    dom.setAttribute('type', 'checkbox');
  }
  if ((semanticKind === 'image' || semanticKind === 'avatar') && !dom.hasAttribute('alt')) {
    const altProp = element.attributes.find((a) => a.name === 'alt');
    dom.setAttribute('alt', altProp && 'Literal' in altProp.value ? String(altProp.value.Literal) : '');
  }
  if (semanticKind === 'editor' && dom.tagName.toLowerCase() === 'textarea' && !dom.hasAttribute('rows')) {
    dom.setAttribute('rows', '4');
  }
  if (semanticKind === 'composer' && dom.tagName.toLowerCase() === 'form' && !dom.hasAttribute('method')) {
    dom.setAttribute('method', 'post');
  }
  if (semanticKind === 'choice' && dom.tagName.toLowerCase() === 'select' && !dom.hasAttribute('multiple')) {
    // single-select by default
  }
  if (semanticKind === 'icon' && dom.tagName.toLowerCase() === 'span' && !dom.hasAttribute('aria-hidden')) {
    const decorative = element.attributes.find((a) => a.name === 'decorative');
    if (decorative && 'Literal' in decorative.value && decorative.value.Literal === 'true') {
      dom.setAttribute('aria-hidden', 'true');
    }
  }
  if (semanticKind === 'media' && dom.tagName.toLowerCase() === 'video' && !dom.hasAttribute('controls')) {
    dom.setAttribute('controls', '');
  }
  if ((semanticKind === 'image' || semanticKind === 'avatar' || semanticKind === 'media') && !dom.hasAttribute('decoding')) {
    dom.setAttribute('decoding', 'async');
  }
  if (semanticKind === 'list' && dom.tagName.toLowerCase() === 'ul' && !dom.hasAttribute('role')) {
    // Native ul has implicit list role; avoid redundant role
  }
  if (semanticKind === 'field' && dom.tagName.toLowerCase() === 'div' && !dom.hasAttribute('role')) {
    // Field is a neutral wrapper; if it has a label, expose it as group
    const label = element.attributes.find((a) => a.name === 'label');
    if (label) {
      dom.setAttribute('role', 'group');
    }
  }
}

function renderComponentInvocation(
  name: string,
  args: readonly { name: string; value: FrameIrComponentArgumentValue }[],
  parent: RenderContext
): RenderedBlock {
  const component = parent.components.get(name);
  if (!component) {
    throw runtimeError(`Unknown component \`${name}\`.`, parent.component.name);
  }
  const props: Record<string, FrameRuntimeValue> = {};
  for (const arg of args) {
    props[arg.name] = readArgument(arg.value, parent);
  }
  const componentLocalPatches: Array<() => void> = [];

  const context: RenderContext = {
    ...parent,
    component,
    props: initializeProps(component, props),
    state: new FrameStateController(initializeState(component), component.name),
    scope: {},
    localPatches: componentLocalPatches
  };
  for (const arg of args) {
    const updateProp = () => {
      (context.props as Record<string, FrameRuntimeValue>)[arg.name] = readArgument(arg.value, parent);
      for (const patch of componentLocalPatches) {
        patch();
      }
    };
    const deps = argumentDependencies(arg.value);
    const stateDeps = deps.filter((dep) => isStateDependency(dep, parent));
    const localDeps = deps.filter((dep) => !isStateDependency(dep, parent));
    if (stateDeps.length > 0) {
      subscribePatch(parent, stateDeps, `component ${name} props`, undefined, updateProp);
    } else if (localDeps.length > 0) {
      registerLocalPatch(parent, updateProp);
    }
  }
  parent.counters.mounts += 1;
  parent.counters.mountedComponents += 1;
  parent.cleanup.push(() => {
    context.state.dispose();
    parent.counters.mountedComponents = Math.max(0, parent.counters.mountedComponents - 1);
  });
  const rendered = block();
  for (const node of component.nodes) {
    mergeBlock(rendered, renderNode(node, context));
  }
  return rendered;
}

function renderList(list: FrameIrList, context: RenderContext): RenderedBlock {
  const start = context.document.createComment(`frame:list:${list.item}:start`);
  const end = context.document.createComment(`frame:list:${list.item}:end`);
  let items: ListItem[] = [];

  const reconcile = () => {
    const parent = end.parentNode;
    if (!parent) {
      return;
    }
    const nextValues = readListValues(list, context);
    const nextItems: ListItem[] = [];
    const oldByKey = new Map<string, ListItem>();
    if (list.key) {
      for (const item of items) {
        oldByKey.set(item.key, item);
      }
    }

    let before: Node = end;
    for (let index = nextValues.length - 1; index >= 0; index -= 1) {
      const value = nextValues[index] ?? null;
      const key = list.key ? String(readScopedPath(value, stripScope(list.key, list.item))) : String(index);
      const existing = list.key ? oldByKey.get(key) : items[index];
      const item = existing ?? renderListItem(list, value, key, context);
      if (!existing) {
        context.counters.listCreates += 1;
      }
      if (existing) {
        context.counters.listReuses += 1;
        patchListItem(item, list, value, context);
      }
      for (let nodeIndex = item.nodes.length - 1; nodeIndex >= 0; nodeIndex -= 1) {
        const node = item.nodes[nodeIndex]!;
        if (node.nextSibling !== before) {
          parent.insertBefore(node, before);
          if (existing) {
            context.counters.listMoves += 1;
          }
        }
        before = node;
      }
      nextItems.unshift(item);
    }

    for (const old of items) {
      if (!nextItems.includes(old)) {
        disposeListItem(old, context);
      }
    }
    items = nextItems;
    count(context, 'patchedLists', 'patched list item');
  };

  const initialValues = readListValues(list, context);
  items = initialValues.map((value, index) => {
    const key = list.key ? String(readScopedPath(value, stripScope(list.key, list.item))) : String(index);
    return renderListItem(list, value, key, context);
  });

  const rendered = block([start]);
  for (const item of items) {
    rendered.nodes.push(...item.nodes);
  }
  rendered.nodes.push(end);

  if (isStateDependency(list.collection, context)) {
    subscribePatch(context, [list.collection], `list ${list.item}`, list.source, reconcile);
  } else {
    registerLocalPatch(context, reconcile);
  }
  context.cleanup.push(() => {
    for (const item of items) {
      disposeListItem(item, context);
    }
    items = [];
  });
  return rendered;
}

type ListItem = {
  key: string;
  value: FrameRuntimeValue;
  scope: Record<string, FrameRuntimeValue>;
  nodes: Node[];
  cleanup: Array<() => void>;
  patches: Array<() => void>;
};

function renderListItem(
  list: FrameIrList,
  value: FrameRuntimeValue,
  key: string,
  context: RenderContext
): ListItem {
  const scopedContext: RenderContext = {
    ...context,
    cleanup: [],
    scope: { ...context.scope, [list.item]: value },
    localPatches: []
  };
  const localPatches = scopedContext.localPatches ?? [];
  const item: ListItem = {
    key,
    value,
    scope: scopedContext.scope,
    nodes: [],
    cleanup: scopedContext.cleanup,
    patches: localPatches
  };
  for (const child of list.children) {
    const rendered = renderNode(child, scopedContext);
    item.nodes.push(...rendered.nodes);
    item.cleanup.push(...rendered.cleanup);
  }
  context.counters.mounts += 1;
  return item;
}

function patchListItem(
  item: ListItem,
  list: FrameIrList,
  value: FrameRuntimeValue,
  context: RenderContext
): void {
  if (Object.is(item.value, value)) {
    return;
  }
  item.value = value;
  item.scope[list.item] = value;
  for (const patch of item.patches) {
    patch();
  }
  count(context, 'patchedLists', 'patched list item');
}

function disposeListItem(item: ListItem, context: RenderContext): void {
  item.cleanup.splice(0).forEach((dispose) => dispose());
  for (const node of item.nodes) {
    if (node.parentNode) {
      node.parentNode.removeChild(node);
      context.counters.disposedNodes += 1;
    }
  }
  context.counters.unmounts += 1;
  context.counters.listRemoves += 1;
}

function readListValues(list: FrameIrList, context: RenderContext): FrameRuntimeValue[] {
  const collection = readValue(list.collection, context);
  if (!Array.isArray(collection)) {
    throw runtimeError(`List source \`$${list.collection}\` is not a list.`, context.component.name, list.source);
  }
  return collection;
}

function applyAttributes(
  dom: Element,
  attributes: readonly FrameIrAttribute[],
  context: RenderContext
): void {
  for (const attribute of attributes) {
    const patch = () => {
      setDomValue(dom, attribute.name, readAttribute(attribute.value, context));
      count(context, 'patchedAttributes', 'patched attribute');
    };
    patchWithoutCount(patch);
    const deps = attributeDependencies(attribute).filter((dep) => isStateDependency(dep, context));
    const localDeps = attributeDependencies(attribute).filter((dep) => !isStateDependency(dep, context));
    if (deps.length > 0) {
      subscribePatch(context, deps, `attribute ${attribute.name}`, attribute.source, patch);
    } else if (localDeps.length > 0) {
      registerLocalPatch(context, patch);
    }
  }
}

function applyBindings(dom: Element, bindings: readonly FrameIrBinding[], context: RenderContext): void {
  for (const binding of bindings) {
    const patch = () => {
      setBindingValue(dom, binding.property, context.state.get(binding.state));
      count(context, 'patchedProperties', 'patched binding');
    };
    patchWithoutCount(patch);
    subscribePatch(context, [binding.state], `binding ${binding.property}`, binding.source, patch);

    const eventName = binding.property === 'value' ? 'input' : 'change';
    const listener = () => {
      if (binding.property === 'checked' && isCheckable(dom)) {
        context.state.set(binding.state, dom.checked);
      } else if (binding.property === 'selected' && isSelectable(dom)) {
        context.state.set(binding.state, dom.value);
      } else if ('value' in dom) {
        context.state.set(binding.state, String(dom.value));
      }
    };
    registerDomListener(dom, eventName, listener, {}, context);
  }
}

function applyConditionalProperties(
  dom: Element,
  conditions: readonly FrameIrCondition[],
  context: RenderContext
): void {
  for (const condition of conditions) {
    if ('Hidden' in condition) {
      const patch = () => {
        (dom as HTMLElement).hidden = Boolean(readValue(condition.Hidden.state, context));
        count(context, 'patchedConditions', 'patched condition');
      };
      patchWithoutCount(patch);
      if (isStateDependency(condition.Hidden.state, context)) {
        subscribePatch(context, [condition.Hidden.state], 'hidden condition', condition.Hidden.source, patch);
      } else {
        registerLocalPatch(context, patch);
      }
    }
    if ('Property' in condition) {
      const patch = () => {
        setBooleanProperty(dom, booleanPropertyName(condition.Property.property), Boolean(readValue(condition.Property.state, context)));
        count(context, 'patchedAttributes', 'patched attribute');
      };
      patchWithoutCount(patch);
      if (isStateDependency(condition.Property.state, context)) {
        subscribePatch(context, [condition.Property.state], `property ${condition.Property.property}`, condition.Property.source, patch);
      } else {
        registerLocalPatch(context, patch);
      }
    }
  }
}

function applyShow(dom: Element, conditions: readonly FrameIrCondition[], context: RenderContext): void {
  for (const condition of conditions) {
    if ('Show' in condition) {
      const patch = () => {
        (dom as HTMLElement).hidden = !Boolean(readValue(condition.Show.state, context));
        count(context, 'patchedConditions', 'patched condition');
      };
      patchWithoutCount(patch);
      if (isStateDependency(condition.Show.state, context)) {
        subscribePatch(context, [condition.Show.state], 'show condition', condition.Show.source, patch);
      } else {
        registerLocalPatch(context, patch);
      }
    }
  }
}

function applyStyles(
  dom: Element,
  style: FrameIrStyleBinding,
  conditions: readonly FrameIrCondition[],
  context: RenderContext
): void {
  dom.classList.add(className(styleName(style)));
  for (const condition of conditions) {
    if ('Style' in condition) {
      const classValue = className(condition.Style.style);
      const patch = () => {
        dom.classList.toggle(classValue, Boolean(readValue(condition.Style.state, context)));
        count(context, 'patchedStyles', 'patched style');
      };
      patchWithoutCount(patch);
      if (isStateDependency(condition.Style.state, context)) {
        subscribePatch(context, [condition.Style.state], `style ${condition.Style.style}`, condition.Style.source, patch);
      } else {
        registerLocalPatch(context, patch);
      }
    }
  }
}

function attachEvents(dom: Element, events: readonly FrameIrEvent[], context: RenderContext): void {
  for (const event of events) {
    const listener = (domEvent: Event) => {
      if (!matchesModifiers(domEvent, event.modifiers)) {
        return;
      }
      if (event.modifiers.includes('prevent') && !event.modifiers.includes('passive')) {
        domEvent.preventDefault();
      }
      if (event.modifiers.includes('stop')) {
        domEvent.stopPropagation();
      }
      const handler = context.handlers[event.handler];
      if (!handler) {
        recordRuntimeError(context, runtimeError(`Missing handler \`@${event.handler}\`.`, context.component.name, event.source));
        return;
      }
      try {
        const result = handler({
          event: domEvent,
          state: context.state,
          props: context.props
        });
        if (isPromiseLike(result)) {
          result.catch((error: unknown) => {
            recordRuntimeError(context, runtimeError(`Handler \`@${event.handler}\` failed.`, context.component.name, event.source, error));
          });
        }
      } catch (error) {
        recordRuntimeError(context, runtimeError(`Handler \`@${event.handler}\` failed.`, context.component.name, event.source, error));
      }
    };
    const options = eventOptions(event.modifiers);
    registerDomListener(dom, domEventName(event.event), listener, options, context);
  }
}

function domEventName(eventName: string): string {
  if (eventName === 'press') {
    return 'click';
  }
  if (eventName === 'send') {
    return 'submit';
  }
  return eventName;
}

function shouldRender(conditions: readonly FrameIrCondition[], context: RenderContext): boolean {
  return conditions.every((condition) => !('Show' in condition) || Boolean(readValue(condition.Show.state, context)));
}

function readText(value: FrameIrTextValue, context: RenderContext): FrameRuntimeValue {
  if ('Literal' in value) {
    return value.Literal;
  }
  return readValue(value.DataRef, context);
}

function readAttribute(value: FrameIrAttributeValue, context: RenderContext): FrameRuntimeValue {
  if ('Literal' in value) {
    return value.Literal;
  }
  return readValue(value.DataRef, context);
}

function readArgument(value: FrameIrComponentArgumentValue, context: RenderContext): FrameRuntimeValue {
  if ('Literal' in value) {
    return value.Literal;
  }
  if ('DataRef' in value) {
    return readValue(value.DataRef, context);
  }
  return readValue(value.Bind, context);
}

function readValue(name: string, context: RenderContext): FrameRuntimeValue {
  if (Object.hasOwn(context.scope, rootName(name))) {
    return readPath(context.scope, name);
  }
  if (Object.hasOwn(context.props, rootName(name))) {
    return readPath(context.props, name);
  }
  return context.state.get(name);
}

function setDomValue(dom: Element, name: string, value: FrameRuntimeValue, propertyOnly = false): void {
  name = domAttributeName(dom, name);
  if (name === 'class' && !propertyOnly) {
    setUserClasses(dom, value);
    return;
  }

  if (!propertyOnly && isUrlAttribute(name)) {
    assertSafeUrlAttribute(name, value);
  }

  if (!propertyOnly) {
    if (value === false || value === null) {
      dom.removeAttribute(name);
    } else if (value === true) {
      dom.setAttribute(name, '');
    } else {
      dom.setAttribute(name, String(value));
    }
  }

  if (name in dom) {
    const selection = captureSelection(dom);
    try {
      (dom as unknown as Record<string, FrameRuntimeValue>)[name] = value;
    } catch {
      // Some DOM properties are readonly in some environments. Attribute writes still carry intent.
    }
    restoreSelection(dom, selection);
  }
}

function setBooleanProperty(dom: Element, name: string, value: FrameRuntimeValue): void {
  const bool = Boolean(value);
  (dom as unknown as Record<string, boolean>)[name] = bool;
  if (bool) {
    dom.setAttribute(name, '');
  } else {
    dom.removeAttribute(name);
  }
}

function booleanPropertyName(name: string): string {
  if (name === 'readonly') return 'readOnly';
  if (name === 'disabled') return 'disabled';
  if (name === 'required') return 'required';
  if (name === 'checked') return 'checked';
  if (name === 'selected') return 'selected';
  if (name === 'multiple') return 'multiple';
  if (name === 'autofocus') return 'autofocus';
  if (name === 'autoplay') return 'autoplay';
  if (name === 'loop') return 'loop';
  if (name === 'muted') return 'muted';
  if (name === 'hidden') return 'hidden';
  return name;
}

function domAttributeName(dom: Element, name: string): string {
  if (name === 'goto') {
    return 'href';
  }
  if (name === 'source' && ['img', 'source', 'video', 'audio'].includes(dom.tagName.toLowerCase())) {
    return 'src';
  }
  if (name === 'label') {
    return 'aria-label';
  }
  return name;
}

function setBindingValue(dom: Element, property: string, value: FrameRuntimeValue): void {
  if (property === 'selected' && isSelectable(dom)) {
    setDomValue(dom, 'value', value, true);
    return;
  }
  setDomValue(dom, property, value, true);
}

function matchesModifiers(event: Event, modifiers: readonly string[]): boolean {
  for (const modifier of modifiers) {
    if (['prevent', 'stop', 'once', 'capture', 'passive'].includes(modifier)) {
      continue;
    }
    if (modifier === 'enter' && (!isKeyboardEvent(event) || event.key !== 'Enter')) {
      return false;
    }
    if (modifier === 'escape' && (!isKeyboardEvent(event) || event.key !== 'Escape')) {
      return false;
    }
    if (modifier === 'space' && (!isKeyboardEvent(event) || event.key !== ' ')) {
      return false;
    }
    if (modifier === 'tab' && (!isKeyboardEvent(event) || event.key !== 'Tab')) {
      return false;
    }
    if (modifier === 'left' && (!isKeyboardEvent(event) || event.key !== 'ArrowLeft')) {
      return false;
    }
    if (modifier === 'right' && (!isKeyboardEvent(event) || event.key !== 'ArrowRight')) {
      return false;
    }
    if (modifier === 'up' && (!isKeyboardEvent(event) || event.key !== 'ArrowUp')) {
      return false;
    }
    if (modifier === 'down' && (!isKeyboardEvent(event) || event.key !== 'ArrowDown')) {
      return false;
    }
    if (modifier === 'ctrl' && (!isKeyboardEvent(event) || !event.ctrlKey)) {
      return false;
    }
    if (modifier === 'shift' && (!isKeyboardEvent(event) || !event.shiftKey)) {
      return false;
    }
    if (modifier === 'alt' && (!isKeyboardEvent(event) || !event.altKey)) {
      return false;
    }
    if (modifier === 'meta' && (!isKeyboardEvent(event) || !event.metaKey)) {
      return false;
    }
  }
  return true;
}

function registerDomListener(
  dom: Element,
  eventName: string,
  listener: EventListener,
  options: AddEventListenerOptions,
  context: RenderContext
): void {
  let active = true;
  const wrapped: EventListener = (event) => {
    if (options.once && active) {
      active = false;
      context.counters.activeListeners = Math.max(0, context.counters.activeListeners - 1);
    }
    listener(event);
  };
  dom.addEventListener(eventName, wrapped, options);
  context.counters.activeListeners += 1;
  context.cleanup.push(() => {
    dom.removeEventListener(eventName, wrapped, options);
    if (active) {
      active = false;
      context.counters.activeListeners = Math.max(0, context.counters.activeListeners - 1);
    }
  });
}

function isKeyboardEvent(event: Event): event is KeyboardEvent {
  return 'key' in event;
}

function isPromiseLike(value: unknown): value is PromiseLike<unknown> & { catch(callback: (error: unknown) => void): unknown } {
  return Boolean(value && typeof value === 'object' && 'then' in value && 'catch' in value);
}

function isCheckable(dom: Element): dom is HTMLInputElement {
  return dom instanceof dom.ownerDocument.defaultView!.HTMLInputElement && dom.type === 'checkbox';
}

function isSelectable(dom: Element): dom is HTMLSelectElement {
  return dom instanceof dom.ownerDocument.defaultView!.HTMLSelectElement;
}

function eventOptions(modifiers: readonly string[]): AddEventListenerOptions {
  return {
    capture: modifiers.includes('capture'),
    once: modifiers.includes('once'),
    passive: modifiers.includes('passive')
  };
}

function setUserClasses(dom: Element, value: FrameRuntimeValue): void {
  const previous = USER_CLASSES.get(dom) ?? new Set<string>();
  for (const className of previous) {
    dom.classList.remove(className);
  }

  const next = new Set(
    String(value ?? '')
      .split(/\s+/)
      .map((className) => className.trim())
      .filter((className) => className.length > 0 && !className.startsWith('fr-'))
  );
  for (const className of next) {
    dom.classList.add(className);
  }
  USER_CLASSES.set(dom, next);
}

function isUrlAttribute(name: string): boolean {
  return ['href', 'src', 'srcset', 'poster', 'action', 'formaction'].includes(name);
}

function assertSafeUrlAttribute(name: string, value: FrameRuntimeValue): void {
  if (value === null || value === false || value === true) {
    return;
  }
  const raw = String(value);
  const values = name === 'srcset'
    ? raw.split(',').map((candidate) => candidate.trim().split(/\s+/)[0] ?? '')
    : [raw];
  if (values.some((candidate) => /^\s*javascript:/i.test(candidate))) {
    throw new FrameDomError(`Unsafe URL scheme for \`${name}\`: javascript URLs are rejected by default.`);
  }
}

function styleName(style: FrameIrStyleBinding): string {
  if ('Explicit' in style) {
    return style.Explicit.style;
  }
  return style.Automatic.style;
}

function className(style: string): string {
  return `fr-${style}`;
}

function appendBlock(parent: Node, rendered: RenderedBlock): void {
  for (const node of rendered.nodes) {
    parent.appendChild(node);
  }
}

function block(nodes: Node[] = []): RenderedBlock {
  return { nodes, cleanup: [] };
}

function mergeBlock(target: RenderedBlock, next: RenderedBlock): void {
  target.nodes.push(...next.nodes);
  target.cleanup.push(...next.cleanup);
}

function subscribePatch(
  context: RenderContext,
  dependencies: string[],
  label: string,
  source: { start: number; end: number } | undefined,
  patch: () => void
): void {
  const scheduledPatch = context.scheduler.createPatch(label, context.component.name, source, patch);
  const unsubscribe = context.state.subscribeTo(dependencies, () => context.scheduler.enqueue(scheduledPatch));
  context.counters.activeSubscriptions += 1;
  context.cleanup.push(() => {
    unsubscribe();
    context.counters.activeSubscriptions = Math.max(0, context.counters.activeSubscriptions - 1);
  });
}

function registerLocalPatch(context: RenderContext, patch: () => void): void {
  context.localPatches?.push(patch);
}

function patchWithoutCount(patch: () => void): void {
  patch();
}

function count(context: RenderContext, key: keyof FrameRuntimeCounters, message: string): void {
  context.counters[key] += 1;
  if (context.debug) {
    console.debug(`[Frame Runtime] ${message}`);
  }
}

function runtimeError(
  message: string,
  component?: string,
  source?: { start: number; end: number },
  cause?: unknown
): FrameDomError {
  return new FrameDomError(message, { component, source, cause });
}

function recordRuntimeError(context: RenderContext, error: FrameDomError): void {
  context.counters.runtimeErrors += 1;
  if (context.debug) {
    console.debug(`[Frame Runtime] ${error.message}`);
  }
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function validateHandlers(component: FrameIrComponent, handlers: FrameHandlerMap, debug: boolean): void {
  const required = new Set<string>();
  collectHandlerRefs(component.nodes, required);
  for (const handler of required) {
    if (!handlers[handler]) {
      if (debug) {
        console.debug(`[Frame Runtime] Warning: missing handler \`@${handler}\` for component \`${component.name}\`.`);
      }
    }
  }
}

function collectHandlerRefs(nodes: readonly FrameIrNode[], refs: Set<string>): void {
  for (const node of nodes) {
    if ('Element' in node) {
      for (const event of node.Element.events) {
        refs.add(event.handler);
      }
      collectHandlerRefs(node.Element.children, refs);
    }
    if ('Component' in node) {
      // Component invocations do not declare handlers in the parent;
      // the child component defines its own handler references.
    }
    if ('List' in node) {
      collectHandlerRefs(node.List.children, refs);
    }
  }
}

function validateProps(component: FrameIrComponent, props: Record<string, FrameRuntimeValue>): void {
  for (const descriptor of component.props) {
    const value = props[descriptor.name];
    if (value === undefined || value === null) {
      continue;
    }
    const expected = descriptor.value_type;
    const actual = typeof value;
    if (expected === 'Bool' && actual !== 'boolean') {
      throw new FrameDomError(
        `Prop \`${descriptor.name}\` expects Bool but received ${actual}.`,
        { component: component.name }
      );
    }
    if (expected === 'Number' && actual !== 'number') {
      throw new FrameDomError(
        `Prop \`${descriptor.name}\` expects Number but received ${actual}.`,
        { component: component.name }
      );
    }
    if (expected === 'Text' && actual !== 'string') {
      throw new FrameDomError(
        `Prop \`${descriptor.name}\` expects Text but received ${actual}.`,
        { component: component.name }
      );
    }
  }
}

function emptyCounters(): FrameRuntimeCounters {
  return {
    mounts: 0,
    unmounts: 0,
    mountedComponents: 0,
    patchedTexts: 0,
    patchedAttributes: 0,
    patchedProperties: 0,
    patchedConditions: 0,
    patchedStyles: 0,
    patchedLists: 0,
    queuedPatches: 0,
    flushedPatches: 0,
    activeListeners: 0,
    activeSubscriptions: 0,
    disposedNodes: 0,
    listMoves: 0,
    listReuses: 0,
    listCreates: 0,
    listRemoves: 0,
    runtimeErrors: 0
  };
}

function attributeDependencies(attribute: FrameIrAttribute): string[] {
  return 'DataRef' in attribute.value ? [attribute.value.DataRef] : [];
}

function argumentDependencies(value: FrameIrComponentArgumentValue): string[] {
  if ('DataRef' in value) {
    return [value.DataRef];
  }
  if ('Bind' in value) {
    return [value.Bind];
  }
  return [];
}

function conditionDependencies(conditions: readonly FrameIrCondition[], kind?: 'Show'): string[] {
  const deps: string[] = [];
  for (const condition of conditions) {
    if ('Show' in condition && (!kind || kind === 'Show')) {
      deps.push(condition.Show.state);
    }
    if ('Hidden' in condition && !kind) {
      deps.push(condition.Hidden.state);
    }
    if ('Property' in condition && !kind) {
      deps.push(condition.Property.state);
    }
    if ('Style' in condition && !kind) {
      deps.push(condition.Style.state);
    }
  }
  return deps;
}

function isStateDependency(name: string, context: RenderContext): boolean {
  return !Object.hasOwn(context.scope, rootName(name)) && !Object.hasOwn(context.props, rootName(name));
}

function rootName(name: string): string {
  return name.split('.')[0] ?? name;
}

function stripScope(path: string, item: string): string {
  return path === item ? '' : path.startsWith(`${item}.`) ? path.slice(item.length + 1) : path;
}

function readPath(values: Readonly<Record<string, FrameRuntimeValue>>, path: string): FrameRuntimeValue {
  const parts = path.split('.');
  let value: FrameRuntimeValue | undefined = values[parts[0] ?? ''];
  for (const part of parts.slice(1)) {
    value = readScopedPath(value ?? null, part);
  }
  return value ?? null;
}

function writePath(values: Record<string, FrameRuntimeValue>, path: string, next: FrameRuntimeValue): void {
  const parts = path.split('.');
  if (parts.length === 1) {
    values[path] = next;
    return;
  }
  let target = values[parts[0]!] as Record<string, FrameRuntimeValue>;
  for (const part of parts.slice(1, -1)) {
    target = target[part] as Record<string, FrameRuntimeValue>;
  }
  target[parts[parts.length - 1]!] = next;
}

function readScopedPath(value: FrameRuntimeValue, path: string): FrameRuntimeValue {
  if (!path) {
    return value;
  }
  let current: FrameRuntimeValue = value;
  for (const part of path.split('.')) {
    if (current && typeof current === 'object' && !Array.isArray(current)) {
      current = current[part] ?? null;
    } else {
      return null;
    }
  }
  return current;
}

function captureSelection(dom: Element): { start: number | null; end: number | null } | null {
  if (dom instanceof dom.ownerDocument.defaultView!.HTMLInputElement || dom instanceof dom.ownerDocument.defaultView!.HTMLTextAreaElement) {
    return { start: dom.selectionStart, end: dom.selectionEnd };
  }
  return null;
}

function restoreSelection(dom: Element, selection: { start: number | null; end: number | null } | null): void {
  if (!selection) {
    return;
  }
  if (dom instanceof dom.ownerDocument.defaultView!.HTMLInputElement || dom instanceof dom.ownerDocument.defaultView!.HTMLTextAreaElement) {
    try {
      dom.setSelectionRange(selection.start, selection.end);
    } catch {
      // Some input types do not expose text selection.
    }
  }
}

export type {
  FrameIrAttribute,
  FrameIrBinding,
  FrameIrComponent,
  FrameIrDocument,
  FrameIrElement,
  FrameIrEvent,
  FrameIrNode
} from './ir.js';
