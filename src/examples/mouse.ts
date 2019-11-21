import { MouseButton, Enigo } from '..';

const enigo = new Enigo();

enigo.mouseClick(MouseButton.Right);
enigo.mouseMoveRelative(-20, -20);

setTimeout(() => {
  enigo.mouseClick(MouseButton.Left);
}, 1000);
