import {isFail, Status, StatusFail} from "./types";

type PromiseCallback<T> = (value: T | PromiseLike<T>) => void;
type NeonCallback<T> = (status: any) => void;
type JsonReviver = (key: string, value: any) => any;

function buildError(status: StatusFail): Error {
    return new Error(
        (status.error?.code || -1) +
        ": " +
        (status.error?.message || 'Internal Error')
    )
}

function resolveStatus<T>(status: Status<T>, resolve: PromiseCallback<T>, reject: PromiseCallback<Error>): void {
    if (isFail(status)) {
        return reject(buildError(status));
    }

    resolve(status.result);
}

function ensureCalled<T>(status: Status<T>, reject: PromiseCallback<Error>): void {
    if (isFail(status)) {
        return reject(buildError(status));
    }
}

function parseStatus<T>(status: string | undefined, reviver?: JsonReviver): Status<T> {
    return JSON.parse(status || '{"succeeded": false}', reviver)
}

function neonToPromise<T>(
    resolve: PromiseCallback<T>,
    reject: PromiseCallback<Error>,
    reviver?: JsonReviver,
): NeonCallback<T> {
    return (status) => resolveStatus(parseStatus<T>(status, reviver), resolve, reject);
}

/**
 * Wraps a Neon function that uses a channel to return result.
 *
 * ```
 * Example usage:
 * function helloWorld(): Promise<string> {
 *     return neonFrameHandlerCall<string>(addon, "helloWorld", []);
 * }
 * ```
 *
 * @param addon rust lib
 * @param method method exported by Neon
 * @param args args for the call
 * @param reviver optional JSON Reviver to fix returned JSON. Ex. to convert Dates from string to Date object.
 */
export function neonFrameHandlerCall<T>(addon: object, method: string, args?: any[], reviver?: JsonReviver): Promise<T> {
    // @ts-ignore
    let f = addon[method];
    return new Promise((resolve, reject) => {
        try {
            if (typeof f == "function") {
                let result = f.apply(f, (args || []).concat([neonToPromise(resolve, reject, reviver)]));
                let status = parseStatus(result);
                ensureCalled(status, reject);
            } else {
                reject(new Error("-2: Unknown function: " + method))
            }
        } catch (e) {
            // @ts-ignore
            if (e != null && typeof e == "object" && typeof e.message == "string") {
                let message = (e as Error).message;
                if (message.startsWith("internal error in Neon module")) {
                    reject(new Error("-3: " + message))
                } else {
                    reject(new Error("-4: " + message))
                }
            } else {
                reject(e)
            }
        }
    })
}

/**
 * Wraps a Neon function that responds directly with a result.
 *
 * ```
 * Example usage:
 * function helloWorld(): string {
 *     return neonFrameDirectCall<string>(addon, "helloWorld", []);
 * }
 * ```
 *
 * @param addon rust lib
 * @param method method exported by Neon
 * @param args args for the call
 * @param reviver optional JSON Reviver to fix returned JSON. Ex. to convert Dates from string to Date object.
 */
export function neonFrameDirectCall<T>(addon: object, method: string, args?: any[], reviver?: JsonReviver): T {
    // @ts-ignore
    let f = addon[method];
    let status;
    if (typeof f == "function") {
        try {
            let result = f.apply(f, args);
            status = parseStatus(result, reviver);
        } catch (e) {
            // @ts-ignore
            if (e != null && typeof e == "object" && typeof e.message == "string") {
                let message = (e as Error).message;
                if (message.startsWith("internal error in Neon module")) {
                    throw new Error("-3: " + message)
                }
            }
            throw e
        }
    } else {
        throw new Error("-2: Unknown function: " + method)
    }

    if (isFail(status)) {
        throw buildError(status);
    }
    // @ts-ignore
    return status.result;
}