import {no_channel, complex_data, ComplexType, default_channel, one_arg, immediate_error, handler_error, no_channel_error, no_method_err, panic_err} from "./access";

describe('Tests', () => {

    describe("No Channel", () => {
        test('no-arg function return value', async () => {
            const act = no_channel();

            expect(act).toEqual("Hello World");
        });

        test('return complex type', async () => {
            const act = complex_data("Hello World");

            expect(typeof act).toBe("object");
            expect(act).toEqual({foo: "Hello World", bar: 11});

            expect(typeof act.foo).toBe("string");
            expect(typeof act.bar).toBe("number");
        });
    });

    describe("Channel", () => {

        test('no-arg channel return value', async () => {
            const act = await default_channel();

            expect(act).toEqual("Just Working Function");
        });

        test('one arg channel return value', async () => {
            const act = await one_arg("foo");

            expect(act).toEqual("Accept handler at pos 1. Argument: foo");
        });

    });

    describe("Errors", () => {
        test('handle channel immediate error', async () => {
            try {
                const _ = await immediate_error();
                fail("should fail before")
            } catch (e) {
                expect((e as Error).message).toBe("1: Error One")
            }
        });

        test('handle promise error', async () => {
            try {
                const _ = await handler_error();
                fail("should fail before")
            } catch (e) {
                expect((e as Error).message).toBe("1: Error One")
            }
        });

        test('handle no-method error', async () => {
            try {
                const _ = await no_method_err();
                fail("should fail before")
            } catch (e) {
                expect((e as Error).message).toBe("-2: Unknown function: no_method")
            }
        });

        test('handle panic', async () => {
            try {
                const _ = await panic_err();
                fail("should fail before")
            } catch (e) {
                expect((e as Error).message).toBe("-3: internal error in Neon module: Just Panic")
            }
        });
    })


})