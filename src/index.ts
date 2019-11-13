import addon from './native/index.node';

import { version } from './~root/package.json';

console.log(addon.hello());

console.log(version);

export { default } from './lib/add';
