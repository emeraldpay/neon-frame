import {neonFrameDirectCall, neonFrameHandlerCall} from "@emeraldpay/neon-frame";

// eslint-disable-next-line @typescript-eslint/no-var-requires
const addon = require('../index.node');

export function no_channel(): string {
    return neonFrameDirectCall<string>(addon, "no_channel", []);
}

export type ComplexType = {foo: string, bar: number};
export function complex_data(a: string): ComplexType {
    return neonFrameDirectCall<ComplexType>(addon, "complex_data", [a]);
}

export function default_channel(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "default_channel", []);
}

export function one_arg(a: string): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "channel_at_1", [a]);
}

export function immediate_error(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "immediate_err", []);
}

export function no_channel_error(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "no_channel_err", []);
}

export function handler_error(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "handler_err", []);
}

export function no_method_err(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "no_method", []);
}

export function panic_err(): Promise<string> {
    return neonFrameHandlerCall<string>(addon, "always_panic", []);
}