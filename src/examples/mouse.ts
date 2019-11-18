import { MouseButton, mouseClick, mouseMoveToRelative } from '..';

mouseClick(MouseButton.Right);
mouseMoveToRelative(-20, -20);

setTimeout(() => {
  mouseClick(MouseButton.Left);
}, 1000);
