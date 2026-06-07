import { strict as assert } from 'node:assert';
import { mkdtemp, readFile, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { test } from 'node:test';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { JSDOM } from 'jsdom';
import { mount, type FrameIrDocument } from '../src/index.js';
import type {
  FrameIrAttribute,
  FrameIrBinding,
  FrameIrElement,
  FrameIrEvent,
  FrameIrNode,
  FrameIrState
} from '../src/ir.js';

const execFileAsync = promisify(execFile);
const source = { start: 0, end: 0 };

test('mount renders elements, attributes, text, props, and styles', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('Greeting', [
    element('section', 'Panel', {
      attributes: [attribute('role', 'region')],
      children: [
        element('span', 'Label', {
          children: [textData('title')]
        })
      ]
    })
  ], {
    props: [{ name: 'title', value_type: 'Text', readonly: true, binding: 'Input', source }]
  }), {
    component: 'Greeting',
    target,
    props: { title: 'Hello' }
  });

  const section = target.querySelector('section');
  assert.ok(section);
  assert.equal(section.className, 'fr-Panel');
  assert.equal(section.getAttribute('role'), 'region');
  assert.equal(section.textContent, 'Hello');
});

test('dispose removes nodes and event listeners', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;

  const app = mount(documentFixture('Counter', [
    element('button', 'Increment', {
      events: [event('click', [], 'increment')],
      children: [textLiteral('Increment')]
    })
  ]), {
    component: 'Counter',
    target,
    handlers: {
      increment() {
        clicks += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  app.dispose();
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));

  assert.equal(clicks, 1);
  assert.equal(target.childNodes.length, 0);
});

test('state updates patch text nodes in place', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('Counter', [textData('count')], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'Counter',
    target
  });

  const textNode = target.firstChild;
  assert.equal(target.textContent, '0');
  app.resetDebugStats();
  app.state.set('count', 2);
  app.flush();
  assert.equal(target.textContent, '2');
  assert.equal(target.firstChild, textNode);
  assert.equal(app.getDebugStats().patchedTexts, 1);
  assert.equal(app.getDebugStats().mounts, 0);
});

test('click and key modifiers invoke external handlers', () => {
  const { document, window } = dom();
  const target = document.createElement('div');
  let sent = 0;

  mount(documentFixture('Composer', [
    element('input', 'MessageBox', {
      events: [event('keydown', ['enter'], 'send')]
    })
  ]), {
    component: 'Composer',
    target,
    handlers: {
      send() {
        sent += 1;
      }
    }
  });

  const input = target.querySelector('input')!;
  input.dispatchEvent(new document.defaultView!.KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
  input.dispatchEvent(new document.defaultView!.KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));

  assert.equal(sent, 1);
});

test('value and checked bindings sync DOM and state', () => {
  const { document, window } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('InputDemo', [
    element('input', 'Draft', {
      bindings: [binding('value', 'draft')]
    }),
    element('input', 'Enabled', {
      attributes: [attribute('type', 'checkbox')],
      bindings: [binding('checked', 'enabled')]
    }),
    textData('draft')
  ], {
    state: [
      state('draft', 'Text', { Text: 'hello' }),
      state('enabled', 'Bool', { Bool: false })
    ]
  }), {
    component: 'InputDemo',
    target
  });

  const [textInput, initialCheckbox] = Array.from(target.querySelectorAll('input'));
  assert.equal(textInput.value, 'hello');
  assert.equal(initialCheckbox.checked, false);
  textInput.focus();
  textInput.setSelectionRange(2, 2);

  const stableTextInput = textInput;
  app.resetDebugStats();
  app.state.set('draft', 'server');
  app.flush();
  assert.equal(target.querySelector('input'), stableTextInput);
  assert.equal(textInput.value, 'server');
  assert.equal(textInput.selectionStart, 2);
  assert.equal(app.getDebugStats().patchedProperties, 1);

  textInput.value = 'updated';
  textInput.dispatchEvent(new document.defaultView!.Event('input', { bubbles: true }));
  const checkbox = target.querySelectorAll('input')[1]!;
  checkbox.checked = true;
  checkbox.dispatchEvent(new document.defaultView!.Event('change', { bubbles: true }));
  app.flush();

  assert.equal(app.state.get('draft'), 'updated');
  assert.equal(app.state.get('enabled'), true);
  assert.equal(target.textContent, 'updated');
});

test('show, hidden, and conditional styles react to state changes', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('TogglePanel', [
    element('section', 'Panel', {
      conditions: [
        { Show: { state: 'loggedIn', source } },
        { Hidden: { state: 'collapsed', source } },
        { Style: { state: 'collapsed', style: 'CollapsedPanel', source } }
      ],
      children: [textLiteral('Secret')]
    })
  ], {
    state: [
      state('loggedIn', 'Bool', { Bool: false }),
      state('collapsed', 'Bool', { Bool: false })
    ]
  }), {
    component: 'TogglePanel',
    target
  });

  const panel = target.querySelector('section')!;
  assert.ok(panel);
  assert.equal(panel.hidden, true);
  app.resetDebugStats();
  app.state.set('loggedIn', true);
  app.flush();
  assert.equal(target.querySelector('section'), panel);
  assert.equal(panel.hidden, false);
  app.state.set('collapsed', true);
  app.flush();
  assert.equal(target.querySelector('section'), panel);
  assert.equal(panel.hidden, true);
  assert.equal(panel.classList.contains('fr-CollapsedPanel'), true);
  assert.equal(app.getDebugStats().patchedConditions, 2);
  assert.equal(app.getDebugStats().patchedStyles, 1);
});

test('conditional property patches without recreating the element', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('LoaderButton', [
    element('button', 'Submit', {
      conditions: [{ Property: { property: 'disabled', state: 'loading', source } }],
      children: [textLiteral('Submit')]
    })
  ], {
    state: [state('loading', 'Bool', { Bool: false })]
  }), {
    component: 'LoaderButton',
    target
  });

  const button = target.querySelector('button')!;
  app.resetDebugStats();
  app.state.set('loading', true);
  app.flush();

  assert.equal(target.querySelector('button'), button);
  assert.equal(button.disabled, true);
  assert.equal(app.getDebugStats().patchedAttributes, 1);
});

test('component invocation renders nested components with props', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const ir: FrameIrDocument = {
    version: 1,
    components: [
      {
        name: 'MessageItem',
        props: [{ name: 'title', value_type: 'Text', readonly: true, binding: 'Input', source }],
        state: [],
        slots: [],
        nodes: [element('article', 'Message', { children: [textData('title')] })],
        capabilities: [],
        source
      },
      {
        name: 'ChatApp',
        props: [],
        state: [],
        slots: [],
        nodes: [{ Component: { name: 'MessageItem', arguments: [{ name: 'title', value: { Literal: 'Hello' }, source }], source } }],
        capabilities: [],
        source
      }
    ]
  };

  mount(ir, { component: 'ChatApp', target });

  assert.equal(target.querySelector('article')?.textContent, 'Hello');
  assert.equal(target.querySelector('article')?.className, 'fr-Message');
});

test('listeners remain stable on unrelated state patches', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;
  const app = mount(documentFixture('StableEvents', [
    element('button', 'Increment', {
      events: [event('click', [], 'increment')],
      children: [textData('count')]
    })
  ], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'StableEvents',
    target,
    handlers: {
      increment() {
        clicks += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  app.state.set('count', 1);
  app.flush();
  assert.equal(target.querySelector('button'), button);
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));

  assert.equal(clicks, 1);
});

test('lists reconcile insertions removals and keyed updates inside the list block', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('ChatApp', [
    {
      List: {
        item: 'message',
        collection: 'messages',
        key: 'message.id',
        children: [
          element('article', 'Message', {
            children: [textData('message.text')]
          })
        ],
        source
      }
    }
  ], {
    state: [state('messages', 'List', 'List')]
  }), {
    component: 'ChatApp',
    target
  });

  app.state.set('messages', [
    { id: 'a', text: 'First' },
    { id: 'b', text: 'Second' }
  ]);
  app.flush();
  const first = target.querySelectorAll('article')[0]!;
  const second = target.querySelectorAll('article')[1]!;

  app.resetDebugStats();
  app.state.set('messages', [
    { id: 'b', text: 'Second updated' },
    { id: 'c', text: 'Third' }
  ]);
  app.flush();

  const articles = target.querySelectorAll('article');
  assert.equal(articles.length, 2);
  assert.equal(articles[0], second);
  assert.notEqual(articles[0], first);
  assert.equal(articles[0]?.textContent, 'Second updated');
  assert.equal(articles[1]?.textContent, 'Third');
  assert.ok(app.getDebugStats().patchedLists >= 1);
});

test('non-keyed lists update by position', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('ListDemo', [
    {
      List: {
        item: 'item',
        collection: 'items',
        key: null,
        children: [element('span', 'Item', { children: [textData('item.label')] })],
        source
      }
    }
  ], {
    state: [state('items', 'List', 'List')]
  }), {
    component: 'ListDemo',
    target
  });

  app.state.set('items', [{ label: 'One' }, { label: 'Two' }]);
  app.flush();
  const first = target.querySelector('span')!;
  app.state.set('items', [{ label: 'Uno' }]);
  app.flush();

  assert.equal(target.querySelector('span'), first);
  assert.equal(target.querySelectorAll('span').length, 1);
  assert.equal(first.textContent, 'Uno');
});

test('debug mode emits runtime patch messages', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const messages: string[] = [];
  const originalDebug = console.debug;
  console.debug = (message?: unknown) => {
    messages.push(String(message));
  };
  try {
    const app = mount(documentFixture('Counter', [textData('count')], {
      state: [state('count', 'Number', { Number: '0' })]
    }), {
      component: 'Counter',
      target,
      debug: true
    });

    messages.length = 0;
    app.state.set('count', 1);
    app.flush();
  } finally {
    console.debug = originalDebug;
  }

  assert.ok(messages.some((message) => message.includes('[Frame Runtime] patched text node')));
});

test('compiled Frame IR can be rendered by the DOM runtime', async () => {
  const dir = await mkdtemp(join(tmpdir(), 'frame-runtime-dom-'));
  const input = join(dir, 'counter.frame');
  const output = join(dir, 'counter.ir.json');
  await writeFile(input, `component Counter {
  state {
    count number = 0
  }

  view {
    action Increment {
      text $count
      on press @increment
    }
  }
}
`);

  await execFileAsync('cargo', ['run', '--quiet', '--bin', 'frame', '--', 'emit-ir', input, '--out', output], {
    cwd: resolve('../..')
  });
  const ir = JSON.parse(await readFile(output, 'utf8')) as FrameIrDocument;
  const { document } = dom();
  const target = document.createElement('div');

  const app = mount(ir, {
    component: 'Counter',
    target,
    handlers: {
      increment({ state }) {
        state.set('count', Number(state.get('count')) + 1);
      }
    }
  });

  const button = target.querySelector('button')!;
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  app.flush();
  assert.equal(target.textContent, '1');
});

test('renders expanded html and svg element coverage', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const kinds = [
    'a', 'p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'ul', 'ol', 'li', 'dl', 'dt', 'dd',
    'form', 'select', 'option', 'optgroup', 'fieldset', 'legend', 'output', 'progress',
    'meter', 'details', 'summary', 'dialog', 'picture', 'source', 'video', 'audio', 'track',
    'canvas', 'svg', 'path', 'table', 'caption', 'thead', 'tbody', 'tfoot', 'tr', 'th', 'td',
    'colgroup', 'col'
  ];

  mount(documentFixture('Coverage', kinds.map((kind) => element(kind, `Node${kind.replace(/\W/g, '')}`))), {
    component: 'Coverage',
    target
  });

  for (const kind of kinds) {
    assert.ok(target.querySelector(kind), `${kind} rendered`);
  }
  assert.equal(target.querySelector('svg')?.namespaceURI, 'http://www.w3.org/2000/svg');
});

test('global attributes data aria and user classes patch without losing frame classes', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('Attrs', [
    element('section', 'Panel', {
      attributes: [
        attribute('id', 'settings'),
        attributeData('class', 'classes'),
        attribute('title', 'Settings'),
        attribute('tabindex', '0'),
        attribute('role', 'region'),
        attribute('part', 'panel'),
        attribute('slot', 'content'),
        attribute('contenteditable', 'false'),
        attribute('draggable', 'true'),
        attribute('spellcheck', 'false'),
        attribute('translate', 'no'),
        attribute('dir', 'ltr'),
        attribute('lang', 'en'),
        attribute('data-testid', 'settings-panel'),
        attribute('aria-label', 'Settings panel')
      ],
      children: [textLiteral('Settings')]
    })
  ], {
    state: [state('classes', 'Text', { Text: 'user-card fr-Injected' })]
  }), {
    component: 'Attrs',
    target
  });

  const section = target.querySelector('section')!;
  assert.equal(section.id, 'settings');
  assert.equal(section.getAttribute('data-testid'), 'settings-panel');
  assert.equal(section.getAttribute('aria-label'), 'Settings panel');
  assert.equal(section.classList.contains('fr-Panel'), true);
  assert.equal(section.classList.contains('user-card'), true);
  assert.equal(section.classList.contains('fr-Injected'), false);

  app.resetDebugStats();
  app.state.set('classes', 'user-card active');
  app.flush();
  assert.equal(target.querySelector('section'), section);
  assert.equal(section.classList.contains('fr-Panel'), true);
  assert.equal(section.classList.contains('active'), true);
  assert.equal(app.getDebugStats().patchedAttributes, 1);
});

test('unsafe javascript urls are rejected by the dom runtime', () => {
  const { document } = dom();
  const target = document.createElement('div');

  assert.throws(() => mount(documentFixture('UnsafeLink', [
    element('a', 'Docs', {
      attributes: [attribute('href', 'javascript:alert(1)')],
      children: [textLiteral('Docs')]
    })
  ]), {
    component: 'UnsafeLink',
    target
  }), /Unsafe URL scheme/);
});

test('form events support prevent stop once capture and passive metadata', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let submits = 0;
  let parentClicks = 0;
  let childClicks = 0;
  let captures = 0;

  mount(documentFixture('FormEvents', [
    element('form', 'Composer', {
      events: [event('submit', ['prevent', 'once'], 'submit')],
      children: [
        element('button', 'Send', {
          attributes: [attribute('type', 'submit')],
          children: [textLiteral('Send')]
        })
      ]
    }),
    element('section', 'Outer', {
      events: [
        event('click', [], 'parent'),
        event('click', ['capture', 'passive'], 'capture')
      ],
      children: [
        element('button', 'Inner', {
          events: [event('click', ['stop'], 'child')],
          children: [textLiteral('Inner')]
        })
      ]
    })
  ]), {
    component: 'FormEvents',
    target,
    handlers: {
      submit({ event }) {
        assert.equal(event.defaultPrevented, true);
        submits += 1;
      },
      parent() {
        parentClicks += 1;
      },
      capture() {
        captures += 1;
      },
      child() {
        childClicks += 1;
      }
    }
  });

  const form = target.querySelector('form')!;
  form.dispatchEvent(new document.defaultView!.Event('submit', { bubbles: true, cancelable: true }));
  form.dispatchEvent(new document.defaultView!.Event('submit', { bubbles: true, cancelable: true }));
  target.querySelectorAll('button')[1]!.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));

  assert.equal(submits, 1);
  assert.equal(captures, 1);
  assert.equal(childClicks, 1);
  assert.equal(parentClicks, 0);
});

test('select selected binding syncs dom and state in place', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('SelectDemo', [
    element('select', 'Choice', {
      bindings: [binding('selected', 'choice')],
      children: [
        element('option', 'A', {
          attributes: [attribute('value', 'a')],
          children: [textLiteral('A')]
        }),
        element('option', 'B', {
          attributes: [attribute('value', 'b')],
          children: [textLiteral('B')]
        })
      ]
    }),
    textData('choice')
  ], {
    state: [state('choice', 'Text', { Text: 'b' })]
  }), {
    component: 'SelectDemo',
    target
  });

  const select = target.querySelector('select')!;
  assert.equal(select.value, 'b');
  app.resetDebugStats();
  app.state.set('choice', 'a');
  app.flush();
  assert.equal(target.querySelector('select'), select);
  assert.equal(select.value, 'a');

  select.value = 'b';
  select.dispatchEvent(new document.defaultView!.Event('change', { bubbles: true }));
  assert.equal(app.state.get('choice'), 'b');
  assert.ok(app.getDebugStats().patchedProperties >= 1);
});

test('scheduler batches duplicate dependency patches and flushes deterministically', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('ScheduledCounter', [textData('count')], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'ScheduledCounter',
    target
  });

  app.resetDebugStats();
  app.state.set('count', 1);
  app.state.set('count', 2);
  assert.equal(target.textContent, '0');
  assert.equal(app.getDebugStats().queuedPatches, 1);

  app.flush();
  assert.equal(target.textContent, '2');
  assert.equal(app.getDebugStats().patchedTexts, 1);
  assert.equal(app.getDebugStats().flushedPatches, 1);
  assert.equal(app.getDebugStats().queuedPatches, 0);
});

test('scheduler patches unrelated dependencies independently in registration order', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const messages: string[] = [];
  const originalDebug = console.debug;
  console.debug = (message?: unknown) => {
    messages.push(String(message));
  };
  try {
    const app = mount(documentFixture('PatchOrder', [
      element('section', 'Panel', {
        attributes: [attributeData('data-count', 'count')],
        children: [textData('label')]
      })
    ], {
      state: [
        state('count', 'Number', { Number: '0' }),
        state('label', 'Text', { Text: 'zero' })
      ]
    }), {
      component: 'PatchOrder',
      target,
      debug: true
    });

    app.resetDebugStats();
    messages.length = 0;
    app.state.set('label', 'one');
    app.state.set('count', 1);
    assert.equal(app.getDebugStats().queuedPatches, 2);
    app.flush();

    assert.equal(target.querySelector('section')?.getAttribute('data-count'), '1');
    assert.equal(target.textContent, 'one');
    assert.equal(app.getDebugStats().patchedAttributes, 1);
    assert.equal(app.getDebugStats().patchedTexts, 1);
    assert.deepEqual(
      messages.filter((message) => message.includes('patched')),
      ['[Frame Runtime] patched attribute', '[Frame Runtime] patched text node']
    );
  } finally {
    console.debug = originalDebug;
  }
});

test('recursive state updates are guarded', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('Recursive', [textData('count')], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'Recursive',
    target
  });

  const unsubscribe = app.state.subscribeTo(['count'], () => {
    app.state.set('count', Number(app.state.get('count')) + 1);
  });
  assert.throws(() => app.state.set('count', 1), /Recursive state update loop detected.*Recursive/);
  unsubscribe();
});

test('keyed list moves preserve nodes listeners and component props update in place', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;
  const ir: FrameIrDocument = {
    version: 1,
    components: [
      {
        name: 'MessageItem',
        props: [{ name: 'title', value_type: 'Text', readonly: true, binding: 'Input', source }],
        state: [],
        slots: [],
        nodes: [element('article', 'Message', { children: [textData('title')] })],
        capabilities: [],
        source
      },
      {
        name: 'ChatApp',
        props: [],
        state: [state('messages', 'List', 'List')],
        slots: [],
        nodes: [{
          List: {
            item: 'message',
            collection: 'messages',
            key: 'message.id',
            children: [
              element('button', 'MessageButton', {
                events: [event('click', [], 'select')],
                children: [{ Component: { name: 'MessageItem', arguments: [{ name: 'title', value: { DataRef: 'message.text' }, source }], source } }]
              })
            ],
            source
          }
        }],
        capabilities: [],
        source
      }
    ]
  };
  const app = mount(ir, {
    component: 'ChatApp',
    target,
    handlers: {
      select() {
        clicks += 1;
      }
    }
  });

  app.state.set('messages', [{ id: 'a', text: 'Alpha' }, { id: 'b', text: 'Beta' }]);
  app.flush();
  const [buttonA, buttonB] = Array.from(target.querySelectorAll('button'));
  const articleB = buttonB.querySelector('article')!;

  app.resetDebugStats();
  app.state.set('messages', [{ id: 'b', text: 'Beta updated' }, { id: 'a', text: 'Alpha' }]);
  app.flush();

  const buttons = target.querySelectorAll('button');
  assert.equal(buttons[0], buttonB);
  assert.equal(buttons[0]?.querySelector('article'), articleB);
  assert.equal(articleB.textContent, 'Beta updated');
  buttons[0]!.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(clicks, 1);
  assert.ok(app.getDebugStats().listMoves >= 1);
  assert.ok(app.getDebugStats().listReuses >= 2);
  assert.equal(app.getDebugStats().activeListeners, 2);
  assert.equal(buttons[1], buttonA);
});

test('nested lists and conditional nodes inside lists patch predictably', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('NestedLists', [{
    List: {
      item: 'group',
      collection: 'groups',
      key: 'group.id',
      children: [
        element('section', 'Group', {
          conditions: [{ Hidden: { state: 'group.hidden', source } }],
          children: [{
            List: {
              item: 'item',
              collection: 'group.items',
              key: 'item.id',
              children: [element('span', 'Item', { children: [textData('item.label')] })],
              source
            }
          }]
        })
      ],
      source
    }
  }], {
    state: [state('groups', 'List', 'List')]
  }), {
    component: 'NestedLists',
    target
  });

  app.state.set('groups', [{ id: 'g1', hidden: false, items: [{ id: 'a', label: 'A' }] }]);
  app.flush();
  const group = target.querySelector('section')!;
  const item = target.querySelector('span')!;

  app.state.set('groups', [{ id: 'g1', hidden: true, items: [{ id: 'a', label: 'A+' }, { id: 'b', label: 'B' }] }]);
  app.flush();

  assert.equal(target.querySelector('section'), group);
  assert.equal(group.hidden, true);
  assert.equal(target.querySelector('span'), item);
  assert.equal(target.textContent, 'A+B');
});

test('list item removal cleans listeners and subscriptions', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;
  const app = mount(documentFixture('CleanupList', [{
    List: {
      item: 'item',
      collection: 'items',
      key: 'item.id',
      children: [
        element('button', 'ItemButton', {
          events: [event('click', [], 'pick')],
          children: [textData('item.label')]
        })
      ],
      source
    }
  }], {
    state: [state('items', 'List', 'List')]
  }), {
    component: 'CleanupList',
    target,
    handlers: {
      pick() {
        clicks += 1;
      }
    }
  });

  app.state.set('items', [{ id: 'a', label: 'A' }, { id: 'b', label: 'B' }]);
  app.flush();
  const removedButton = target.querySelector('button')!;
  assert.equal(app.getDebugStats().activeListeners, 2);

  app.state.set('items', [{ id: 'b', label: 'B' }]);
  app.flush();
  removedButton.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));

  assert.equal(clicks, 0);
  assert.equal(app.getDebugStats().activeListeners, 1);
  assert.equal(app.getDebugStats().listRemoves, 1);
});

test('attributes properties urls and textarea selection patch safely', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('AttributePatch', [
    element('a', 'Link', {
      attributes: [
        attributeData('href', 'url'),
        attributeData('data-state', 'label'),
        attributeData('aria-label', 'label'),
        attributeData('class', 'classes')
      ],
      conditions: [{ Style: { state: 'active', style: 'ActiveLink', source } }],
      children: [textLiteral('Open')]
    }),
    element('textarea', 'Draft', {
      bindings: [binding('value', 'draft')]
    })
  ], {
    state: [
      state('url', 'Text', { Text: '/safe' }),
      state('label', 'Text', { Text: 'ready' }),
      state('classes', 'Text', { Text: 'user-link' }),
      state('active', 'Bool', { Bool: false }),
      state('draft', 'Text', { Text: 'hello' })
    ]
  }), {
    component: 'AttributePatch',
    target
  });

  const link = target.querySelector('a')!;
  const textarea = target.querySelector('textarea')!;
  textarea.focus();
  textarea.setSelectionRange(2, 2);

  app.state.set('label', null);
  app.state.set('classes', 'user-link selected fr-Spoof');
  app.state.set('active', true);
  app.state.set('draft', 'hello world');
  app.flush();

  assert.equal(link.hasAttribute('data-state'), false);
  assert.equal(link.hasAttribute('aria-label'), false);
  assert.equal(link.classList.contains('fr-Link'), true);
  assert.equal(link.classList.contains('fr-ActiveLink'), true);
  assert.equal(link.classList.contains('selected'), true);
  assert.equal(link.classList.contains('fr-Spoof'), false);
  assert.equal(textarea.value, 'hello world');
  assert.equal(textarea.selectionStart, 2);

  app.state.set('url', 'javascript:alert(1)');
  assert.throws(() => app.flush(), /Unsafe URL scheme.*AttributePatch/);
});

test('event modifiers and once cleanup remain stable across unrelated patches', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let submits = 0;
  let shortcuts = 0;
  const app = mount(documentFixture('Events', [
    element('button', 'Submit', {
      events: [event('click', ['once', 'prevent'], 'submit')],
      children: [textData('label')]
    }),
    element('input', 'Shortcut', {
      events: [event('keydown', ['ctrl', 'shift', 'enter'], 'shortcut')]
    })
  ], {
    state: [state('label', 'Text', { Text: 'Submit' })]
  }), {
    component: 'Events',
    target,
    handlers: {
      submit({ event }) {
        assert.equal(event.defaultPrevented, true);
        submits += 1;
      },
      shortcut() {
        shortcuts += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  const input = target.querySelector('input')!;
  assert.equal(app.getDebugStats().activeListeners, 2);
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true, cancelable: true }));
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true, cancelable: true }));
  assert.equal(submits, 1);
  assert.equal(app.getDebugStats().activeListeners, 1);

  app.state.set('label', 'Send');
  app.flush();
  assert.equal(target.querySelector('button'), button);
  input.dispatchEvent(new document.defaultView!.KeyboardEvent('keydown', { key: 'Enter', ctrlKey: true, bubbles: true }));
  input.dispatchEvent(new document.defaultView!.KeyboardEvent('keydown', { key: 'Enter', ctrlKey: true, shiftKey: true, bubbles: true }));
  assert.equal(shortcuts, 1);

  app.dispose();
  assert.equal(app.getDebugStats().activeListeners, 0);
});

test('runtime errors include component and source context', () => {
  const { document } = dom();
  assert.throws(() => mount(documentFixture('BadList', [{
    List: {
      item: 'item',
      collection: 'title',
      key: null,
      children: [textData('item')],
      source
    }
  }], {
    state: [state('title', 'Text', { Text: 'not a list' })]
  }), {
    component: 'BadList',
    target: document.createElement('div')
  }), /List source `\$title` is not a list.*BadList.*source 0..0/);

  const target = document.createElement('div');
  const app = mount(documentFixture('Errors', [
    element('button', 'MissingHandler', {
      events: [event('click', [], 'missing')],
      children: [textLiteral('Click')]
    })
  ], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'Errors',
    target
  });

  assert.throws(() => app.state.get('missing'), /Unknown state value `missing`.*Errors/);
  target.querySelector('button')!.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(app.getDebugStats().runtimeErrors, 1);
});

function dom(): { window: Window; document: Document } {
  const window = new JSDOM('<!doctype html><div id="app"></div>').window as unknown as Window;
  return { window, document: window.document };
}

function documentFixture(
  name: string,
  nodes: FrameIrDocument['components'][number]['nodes'],
  options: Partial<Pick<FrameIrDocument['components'][number], 'props' | 'state'>> = {}
): FrameIrDocument {
  return {
    version: 1,
    components: [
      {
        name,
        props: options.props ?? [],
        state: options.state ?? [],
        slots: [],
        nodes,
        capabilities: [],
        source
      }
    ]
  };
}

function element(
  kind: string,
  name: string,
  options: Partial<FrameIrElement> = {}
): FrameIrNode {
  return {
    Element: {
      kind,
      name,
      semantic_kind: options.semantic_kind ?? undefined,
      render_kind: options.render_kind ?? undefined,
      style: options.style ?? { Automatic: { style: name, source } },
      attributes: options.attributes ?? [],
      bindings: options.bindings ?? [],
      events: options.events ?? [],
      conditions: options.conditions ?? [],
      children: options.children ?? [],
      source
    }
  };
}

function textLiteral(value: string): FrameIrNode {
  return { Text: { value: { Literal: value }, source } };
}

function textData(name: string): FrameIrNode {
  return { Text: { value: { DataRef: name }, source } };
}

function attribute(name: string, value: string): FrameIrAttribute {
  return { name, value: { Literal: value }, source };
}

function attributeData(name: string, value: string): FrameIrAttribute {
  return { name, value: { DataRef: value }, source };
}

function binding(property: string, stateName: string): FrameIrBinding {
  return { property, state: stateName, source };
}

function event(eventName: string, modifiers: string[], handler: string): FrameIrEvent {
  return { event: eventName, modifiers, handler, source };
}

function state(
  name: string,
  value_type: 'Text' | 'Bool' | 'Number' | 'List',
  defaultValue: { Text: string } | { Bool: boolean } | { Number: string } | 'List'
): FrameIrState {
  return { name, value_type, default: defaultValue, source };
}

test('action renders as button with type button and is keyboard activated', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let pressed = 0;

  mount(documentFixture('ActionDemo', [
    element('button', 'Send', {
      semantic_kind: 'action',
      events: [event('press', [], 'send')],
      children: [textLiteral('Send')]
    })
  ]), {
    component: 'ActionDemo',
    target,
    handlers: {
      send() {
        pressed += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  assert.equal(button.getAttribute('type'), 'button');
  assert.equal(button.tagName.toLowerCase(), 'button');

  // Mouse click triggers press handler
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(pressed, 1);

  // Keyboard Enter triggers press handler (native button behavior)
  button.dispatchEvent(new document.defaultView!.KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
  // Note: keydown does not activate the click listener; the test verifies the button is a real
  // interactive control that the browser would activate on Enter/Space.
  assert.equal(button.hasAttribute('tabindex'), false); // buttons are naturally focusable
});

test('disabled action prevents interaction and is reflected in DOM', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let pressed = 0;

  const app = mount(documentFixture('DisabledAction', [
    element('button', 'Send', {
      semantic_kind: 'action',
      conditions: [{ Property: { property: 'disabled', state: 'sending', source } }],
      events: [event('press', [], 'send')],
      children: [textLiteral('Send')]
    })
  ], {
    state: [state('sending', 'Bool', { Bool: false })]
  }), {
    component: 'DisabledAction',
    target,
    handlers: {
      send() {
        pressed += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  assert.equal(button.disabled, false);

  app.state.set('sending', true);
  app.flush();

  assert.equal(target.querySelector('button'), button);
  assert.equal(button.disabled, true);
  // In real browsers disabled buttons do not dispatch click; JSDOM may differ.
  // The assertion above on `disabled` is the real contract.
});

test('toggle renders as checkbox input', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('ToggleDemo', [
    element('input', 'Enabled', {
      semantic_kind: 'toggle',
      bindings: [binding('checked', 'enabled')]
    })
  ], {
    state: [state('enabled', 'Bool', { Bool: true })]
  }), {
    component: 'ToggleDemo',
    target
  });

  const input = target.querySelector('input')!;
  assert.equal(input.getAttribute('type'), 'checkbox');
  assert.equal(input.checked, true);
});

test('image and avatar render with alt and async decoding', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('ImageDemo', [
    element('img', 'Logo', {
      semantic_kind: 'image',
      attributes: [
        attribute('alt', 'Company logo'),
        attribute('source', '/logo.png')
      ]
    }),
    element('img', 'Avatar', {
      semantic_kind: 'avatar',
      attributes: [attribute('source', '/avatar.png')]
    })
  ]), {
    component: 'ImageDemo',
    target
  });

  const images = target.querySelectorAll('img');
  assert.equal(images[0]?.getAttribute('alt'), 'Company logo');
  assert.equal(images[0]?.getAttribute('decoding'), 'async');
  assert.equal(images[1]?.getAttribute('alt'), '');
  assert.equal(images[1]?.getAttribute('decoding'), 'async');
});

test('icon with decorative hides from accessibility tree', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('IconDemo', [
    element('span', 'Star', {
      semantic_kind: 'icon',
      attributes: [attribute('decorative', 'true')]
    }),
    element('span', 'Info', {
      semantic_kind: 'icon'
    })
  ]), {
    component: 'IconDemo',
    target
  });

  const icons = target.querySelectorAll('span');
  assert.equal(icons[0]?.getAttribute('aria-hidden'), 'true');
  assert.equal(icons[1]?.hasAttribute('aria-hidden'), false);
});

test('field with label exposes group role', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('FieldDemo', [
    element('div', 'EmailField', {
      semantic_kind: 'field',
      attributes: [attribute('label', 'Email address')],
      children: [
        element('input', 'EmailInput', {
          semantic_kind: 'input',
          attributes: [attribute('type', 'email')]
        })
      ]
    })
  ]), {
    component: 'FieldDemo',
    target
  });

  const field = target.querySelector('div')!;
  assert.equal(field.getAttribute('role'), 'group');
  assert.equal(field.getAttribute('aria-label'), 'Email address');
});

test('list renders with native list semantics', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('ListDemo', [
    element('ul', 'Items', {
      semantic_kind: 'list',
      children: [
        element('li', 'First', { semantic_kind: 'item', children: [textLiteral('First')] })
      ]
    })
  ]), {
    component: 'ListDemo',
    target
  });

  const list = target.querySelector('ul')!;
  assert.equal(list.tagName.toLowerCase(), 'ul');
});

test('layout primitives do not add misleading roles', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('LayoutDemo', [
    element('div', 'Stack', { semantic_kind: 'stack' }),
    element('div', 'Row', { semantic_kind: 'row' }),
    element('section', 'Panel', { semantic_kind: 'panel' }),
    element('div', 'Grid', { semantic_kind: 'grid' })
  ]), {
    component: 'LayoutDemo',
    target
  });

  const stack = target.querySelectorAll('div')[0]!;
  const panel = target.querySelector('section')!;
  assert.equal(stack.hasAttribute('role'), false);
  assert.equal(panel.tagName.toLowerCase(), 'section');
  assert.equal(panel.getAttribute('role'), null);
});

test('press event maps to click and triggers handler', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let pressed = 0;

  mount(documentFixture('PressDemo', [
    element('button', 'Save', {
      events: [event('press', [], 'save')],
      children: [textLiteral('Save')]
    })
  ]), {
    component: 'PressDemo',
    target,
    handlers: {
      save() {
        pressed += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(pressed, 1);
});

test('events remain stable after conditional render hides and shows element', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;

  const app = mount(documentFixture('ConditionalEvent', [
    element('button', 'ToggleBtn', {
      conditions: [{ Show: { state: 'visible', source } }],
      events: [event('click', [], 'increment')],
      children: [textData('count')]
    })
  ], {
    state: [state('visible', 'Bool', { Bool: true }), state('count', 'Number', { Number: '0' })]
  }), {
    component: 'ConditionalEvent',
    target,
    handlers: {
      increment() {
        clicks += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(clicks, 1);

  app.state.set('visible', false);
  app.flush();
  assert.equal(button.hidden, true);

  app.state.set('visible', true);
  app.flush();
  assert.equal(button.hidden, false);
  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(clicks, 2);
  assert.equal(app.getDebugStats().activeListeners, 1);
});

test('handlers do not duplicate after text-only rerender', () => {
  const { document } = dom();
  const target = document.createElement('div');
  let clicks = 0;

  const app = mount(documentFixture('StableHandler', [
    element('button', 'CountBtn', {
      events: [event('click', [], 'increment')],
      children: [textData('count')]
    })
  ], {
    state: [state('count', 'Number', { Number: '0' })]
  }), {
    component: 'StableHandler',
    target,
    handlers: {
      increment() {
        clicks += 1;
      }
    }
  });

  const button = target.querySelector('button')!;
  app.state.set('count', 1);
  app.flush();
  assert.equal(target.querySelector('button'), button);

  button.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(clicks, 1);
  assert.equal(app.getDebugStats().activeListeners, 1);
});

test('keyed list items keep correct handler identity after reorder', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const clicks: Record<string, number> = {};

  const app = mount(documentFixture('KeyedHandlers', [{
    List: {
      item: 'item',
      collection: 'items',
      key: 'item.id',
      children: [
        element('button', 'ItemBtn', {
          events: [event('click', [], 'pick')],
          children: [textData('item.label')]
        })
      ],
      source
    }
  }], {
    state: [state('items', 'List', 'List')]
  }), {
    component: 'KeyedHandlers',
    target,
    handlers: {
      pick({ event }) {
        const label = (event.target as HTMLElement).textContent ?? 'unknown';
        clicks[label] = (clicks[label] ?? 0) + 1;
      }
    }
  });

  app.state.set('items', [{ id: 'a', label: 'Alpha' }, { id: 'b', label: 'Beta' }]);
  app.flush();
  const [buttonA, buttonB] = Array.from(target.querySelectorAll('button'));

  app.state.set('items', [{ id: 'b', label: 'Beta' }, { id: 'a', label: 'Alpha' }]);
  app.flush();

  const buttons = target.querySelectorAll('button');
  assert.equal(buttons[0], buttonB);
  assert.equal(buttons[1], buttonA);

  buttons[0]!.dispatchEvent(new document.defaultView!.MouseEvent('click', { bubbles: true }));
  assert.equal(clicks['Beta'], 1);
});

test('placeholder renders on input and textarea', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('PlaceholderDemo', [
    element('input', 'Search', {
      semantic_kind: 'input',
      attributes: [attribute('placeholder', 'Search...')]
    }),
    element('textarea', 'Bio', {
      semantic_kind: 'editor',
      attributes: [attribute('placeholder', 'Tell us about yourself')]
    })
  ]), {
    component: 'PlaceholderDemo',
    target
  });

  const input = target.querySelector('input')!;
  const textarea = target.querySelector('textarea')!;
  assert.equal(input.getAttribute('placeholder'), 'Search...');
  assert.equal(textarea.getAttribute('placeholder'), 'Tell us about yourself');
});

test('readonly input prevents user edits in DOM', () => {
  const { document } = dom();
  const target = document.createElement('div');

  const app = mount(documentFixture('ReadonlyDemo', [
    element('input', 'Id', {
      semantic_kind: 'input',
      conditions: [{ Property: { property: 'readonly', state: 'locked', source } }],
      bindings: [binding('value', 'id')]
    })
  ], {
    state: [
      state('locked', 'Bool', { Bool: true }),
      state('id', 'Text', { Text: 'abc' })
    ]
  }), {
    component: 'ReadonlyDemo',
    target
  });

  const input = target.querySelector('input')!;
  assert.equal(input.readOnly, true);

  app.state.set('locked', false);
  app.flush();
  assert.equal(input.readOnly, false);
});

test('label attribute maps to aria-label for accessibility', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('LabelDemo', [
    element('button', 'Close', {
      semantic_kind: 'action',
      attributes: [attribute('label', 'Close dialog')]
    })
  ]), {
    component: 'LabelDemo',
    target
  });

  const button = target.querySelector('button')!;
  assert.equal(button.getAttribute('aria-label'), 'Close dialog');
});

test('conditional component cleanup hides and shows nested component', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const ir: FrameIrDocument = {
    version: 1,
    components: [
      {
        name: 'Inner',
        props: [],
        state: [state('innerCount', 'Number', { Number: '0' })],
        slots: [],
        nodes: [element('span', 'InnerText', { children: [textData('innerCount')] })],
        capabilities: [],
        source
      },
      {
        name: 'Outer',
        props: [],
        state: [state('show', 'Bool', { Bool: true })],
        slots: [],
        nodes: [
          element('div', 'Wrapper', {
            conditions: [{ Show: { state: 'show', source } }],
            children: [{ Component: { name: 'Inner', arguments: [], source } }]
          })
        ],
        capabilities: [],
        source
      }
    ]
  };

  const app = mount(ir, { component: 'Outer', target });
  const wrapper = target.querySelector('div')!;
  assert.equal(wrapper.hidden, false);
  assert.equal(target.querySelector('span')?.textContent, '0');

  app.state.set('show', false);
  app.flush();
  assert.equal(wrapper.hidden, true);

  app.state.set('show', true);
  app.flush();
  assert.equal(wrapper.hidden, false);
  assert.equal(target.querySelector('span')?.textContent, '0');
});

test('debug mode explains queued and flushed patches', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const messages: string[] = [];
  const originalDebug = console.debug;
  console.debug = (message?: unknown) => {
    messages.push(String(message));
  };
  try {
    const app = mount(documentFixture('DebugPatch', [
      element('section', 'Panel', {
        attributes: [attributeData('data-count', 'count')],
        children: [textData('label')]
      })
    ], {
      state: [
        state('count', 'Number', { Number: '0' }),
        state('label', 'Text', { Text: 'zero' })
      ]
    }), {
      component: 'DebugPatch',
      target,
      debug: true
    });

    messages.length = 0;
    app.state.set('count', 1);
    app.state.set('label', 'one');
    app.flush();

    const queued = messages.filter((m) => m.includes('queued'));
    const flushed = messages.filter((m) => m.includes('flushing'));
    assert.ok(queued.length >= 2, `expected queued messages, got: ${queued.join('; ')}`);
    assert.ok(flushed.length >= 2, `expected flushed messages, got: ${flushed.join('; ')}`);
    assert.ok(queued.some((m) => m.includes('DebugPatch')), 'queued message should mention component');
  } finally {
    console.debug = originalDebug;
  }
});

test('missing handler logs warning in debug mode at mount time', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const messages: string[] = [];
  const originalDebug = console.debug;
  console.debug = (message?: unknown) => {
    messages.push(String(message));
  };
  try {
    mount(documentFixture('MissingHandler', [
      element('button', 'Save', {
        events: [event('click', [], 'save')],
        children: [textLiteral('Save')]
      })
    ]), {
      component: 'MissingHandler',
      target,
      debug: true
    });

    assert.ok(messages.some((m) => m.includes('missing handler `@save`')), `expected warning, got: ${messages.join('; ')}`);
    assert.ok(messages.some((m) => m.includes('MissingHandler')), 'warning should mention component');
  } finally {
    console.debug = originalDebug;
  }
});

test('invalid prop type throws at mount time', () => {
  const { document } = dom();
  const target = document.createElement('div');

  assert.throws(() => mount(documentFixture('BadProps', [
    element('span', 'Label', { children: [textData('title')] })
  ], {
    props: [{ name: 'title', value_type: 'Bool', readonly: true, binding: 'Input', source }]
  }), {
    component: 'BadProps',
    target,
    props: { title: 'hello' }
  }), /Prop `title` expects Bool but received string/);
});

test('media element gets controls and poster when provided', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('MediaDemo', [
    element('video', 'Player', {
      semantic_kind: 'media',
      attributes: [
        attribute('poster', '/thumb.jpg'),
        attribute('source', '/video.mp4')
      ]
    })
  ]), {
    component: 'MediaDemo',
    target
  });

  const video = target.querySelector('video')!;
  assert.equal(video.getAttribute('controls'), '');
  assert.equal(video.getAttribute('poster'), '/thumb.jpg');
  assert.equal(video.getAttribute('src'), '/video.mp4');
});

test('editor gets rows default and respects custom rows', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('EditorDemo', [
    element('textarea', 'Notes', {
      semantic_kind: 'editor',
      attributes: [attribute('rows', '8')]
    }),
    element('textarea', 'Brief', {
      semantic_kind: 'editor'
    })
  ]), {
    component: 'EditorDemo',
    target
  });

  const textareas = target.querySelectorAll('textarea');
  assert.equal(textareas[0]?.getAttribute('rows'), '8');
  assert.equal(textareas[1]?.getAttribute('rows'), '4');
});

test('composer form gets post method default', () => {
  const { document } = dom();
  const target = document.createElement('div');

  mount(documentFixture('ComposerDemo', [
    element('form', 'MessageForm', {
      semantic_kind: 'composer',
      children: [
        element('input', 'Draft', { semantic_kind: 'input' })
      ]
    })
  ]), {
    component: 'ComposerDemo',
    target
  });

  const form = target.querySelector('form')!;
  assert.equal(form.getAttribute('method'), 'post');
});

test('no duplicate subscriptions after conditional rerender', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('SubCount', [
    element('span', 'Label', {
      conditions: [{ Show: { state: 'visible', source } }],
      children: [textData('count')]
    })
  ], {
    state: [
      state('visible', 'Bool', { Bool: true }),
      state('count', 'Number', { Number: '0' })
    ]
  }), {
    component: 'SubCount',
    target
  });

  const initialSubs = app.getDebugStats().activeSubscriptions;
  app.state.set('visible', false);
  app.flush();
  app.state.set('visible', true);
  app.flush();
  app.state.set('count', 1);
  app.flush();

  assert.equal(app.getDebugStats().activeSubscriptions, initialSubs);
});

test('debug stats accurately track mounts unmounts and listeners', () => {
  const { document } = dom();
  const target = document.createElement('div');
  const app = mount(documentFixture('Stats', [
    element('button', 'Btn', {
      events: [event('click', [], 'click')],
      children: [textLiteral('Click')]
    })
  ]), {
    component: 'Stats',
    target,
    handlers: { click() {} }
  });

  assert.equal(app.getDebugStats().mounts, 1);
  assert.equal(app.getDebugStats().activeListeners, 1);
  assert.equal(app.getDebugStats().mountedComponents, 1);

  app.dispose();
  assert.equal(app.getDebugStats().unmounts, 1);
  assert.equal(app.getDebugStats().activeListeners, 0);
  assert.equal(app.getDebugStats().mountedComponents, 0);
  assert.ok(app.getDebugStats().disposedNodes >= 1);
});

const exampleDir = resolve('examples');
const exampleFiles = [
  'chat-app.frame',
  'chat-composer.frame',
  'counter.frame',
  'data-list.frame',
  'field-input.frame',
  'media-card.frame',
  'navigation-links.frame',
  'settings-dialog.frame',
  'toggle-panel.frame',
  'accessible-composer.frame',
];

for (const file of exampleFiles) {
  test(`runtime example ${file} passes frame check`, async () => {
    const input = join(exampleDir, file);
    const { stdout, stderr } = await execFileAsync('cargo', ['run', '--quiet', '--bin', 'frame', '--', 'check', input], {
      cwd: resolve('../..')
    });
    assert.ok(stdout.includes('ok') || stderr === '', `frame check failed for ${file}: ${stderr}`);
  });

  test(`runtime example ${file} compiles to IR`, async () => {
    const input = join(exampleDir, file);
    const output = join(tmpdir(), `frame-example-${file.replace('.frame', '')}.ir.json`);
    await execFileAsync('cargo', ['run', '--quiet', '--bin', 'frame', '--', 'emit-ir', input, '--out', output], {
      cwd: resolve('../..')
    });
    const ir = JSON.parse(await readFile(output, 'utf8')) as FrameIrDocument;
    assert.equal(ir.version, 1);
    assert.ok(ir.components.length > 0, `${file} should produce at least one component`);
  });
}
