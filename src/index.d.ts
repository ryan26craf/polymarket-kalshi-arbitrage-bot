export declare function useAsyncState(promiseFn: () => Promise<any>): {
    data: any;
    loading: boolean;
    error: Error | null;
    run: () => Promise<any>;
};