/**
 * Simple hook-like utility for async state.
 * @param {Function} promiseFn 
 */
function useAsyncState(promiseFn) {
    return {
        data: null,
        loading: false,
        error: null,
        run: () => {
            if (typeof promiseFn === 'function') {
                return promiseFn();
            }
        }
    };
}

module.exports = { useAsyncState };