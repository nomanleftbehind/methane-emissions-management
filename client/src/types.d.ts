import type { ControllersByQuery } from './codegen';

export type IInferFromArray<T> = T extends (infer U)[] ? U : never;
export type IController = IInferFromArray<ControllersByQuery['controllersBy']>;
