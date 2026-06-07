import { mount } from '@frame/runtime-dom';
import appIr from './generated/app.ir';
import { handlers } from './handlers';

const app = mount(appIr, {
  component: 'TodoApp',
  target: document.getElementById('app')!,
  handlers,
  debug: new URLSearchParams(window.location.search).has('debug')
});

declare global {
  interface Window {
    frameApp?: typeof app;
  }
}

window.frameApp = app;
