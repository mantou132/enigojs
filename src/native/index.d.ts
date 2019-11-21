/* eslint-disable @typescript-eslint/camelcase */
import { MouseButton, Key } from '../lib/type';

export class Enigo {
  mouseMoveTo(x: number, y: number): void;
  mouseMoveRelative(x: number, y: number): void;
  mouseDown(button: MouseButton): void;
  mouseUp(button: MouseButton): void;
  mouseClick(button: MouseButton): void;
  keyDown(key: Key): void;
  keyUp(key: Key): void;
  keyClick(key: Key): void;
  keySequence(keySequence: string): void;
  keySequenceParse(keySequence: string): void;
}
