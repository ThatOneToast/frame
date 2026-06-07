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
