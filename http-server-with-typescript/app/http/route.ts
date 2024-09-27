import { MethodType, Method } from "./method";

export interface IRoute {
  method: MethodType;
  pattern: string;
  callback: Function;
}

const DUMMY_CALLBACK = () => {};

class Route implements IRoute {
  method: MethodType;
  pattern: string;
  callback: Function;

  constructor(method: MethodType, pattern: string, callback: Function) {
    this.method = method || Method.GET;
    this.pattern = pattern || "/";
    this.callback = callback || DUMMY_CALLBACK;
  }
}

export default Route;
