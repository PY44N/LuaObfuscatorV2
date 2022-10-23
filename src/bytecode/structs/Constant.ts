import { LuaType } from "../enums/LuaType";

export class Constant {
  type: LuaType;
  data: null | boolean | number | string;

  constructor(type: LuaType) {
    this.type = type;
    this.data = null;
  }
}
