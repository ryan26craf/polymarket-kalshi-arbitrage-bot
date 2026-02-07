const { useAsyncState } = require('../src/index');

describe('useAsyncState', () => {
    test('should return initial state', () => {
        const state = useAsyncState(() => Promise.resolve('ok'));
        expect(state).toHaveProperty('data', null);
        expect(state).toHaveProperty('loading', false);
        expect(state).toHaveProperty('error', null);
    });

    test('run should execute promise', async () => {
        const fn = jest.fn().mockResolvedValue('ok');
        const state = useAsyncState(fn);
        await state.run();
        expect(fn).toHaveBeenCalled();
    });
});