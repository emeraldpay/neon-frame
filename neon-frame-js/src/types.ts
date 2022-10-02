export type StatusOk<T> = {
    succeeded: true;
    /**
     * Actual result (as from `Ok(result)` in Rust)
     */
    result: T;
};

export type StatusFail = {
    succeeded: false;
    /**
     * Error (as from `Err(usize, String)` in Rust)
     */
    error: {
        code: number;
        message: string;
    };
};

/**
 * Standard type for wrapped data returned from Neon Frame
 */
export type Status<T> = StatusOk<T> | StatusFail;

export function isFail(status: Status<any>): status is StatusFail {
    // @ts-ignore
    return typeof status == "object" && !status.succeeded && typeof (status.error) == "object"
}