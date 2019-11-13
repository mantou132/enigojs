import { MouseButton, Key } from '../lib/type';

export function mouseMoveTo(x: number, y: number): void;
export function mouseMoveToRelative(x: number, y: number): void;
export function keyDown(key: Key): void;
export function keyUp(key: Key): void;
export function keyClick(key: Key): void;
export function keySequence(keySequence: string): void;
export function keySequenceParse(keySequence: string): void;
export function mouseDown(button: MouseButton): void;
export function mouseUp(button: MouseButton): void;
export function mouseClick(button: MouseButton): void;
