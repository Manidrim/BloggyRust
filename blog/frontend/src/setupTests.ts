import "@testing-library/jest-dom";

// Node v25+ ships a native localStorage that lacks .clear() and other standard
// Storage methods. Replace it with a fully-spec-compliant in-memory mock so that
// tests (and the app code that uses localStorage) work correctly.
if (typeof localStorage !== "undefined" && typeof localStorage.clear !== "function") {
  const store: Record<string, string> = {};
  const mockStorage: Storage = {
    get length() {
      return Object.keys(store).length;
    },
    key(index: number) {
      return Object.keys(store)[index] ?? null;
    },
    getItem(key: string) {
      return Object.prototype.hasOwnProperty.call(store, key) ? store[key] : null;
    },
    setItem(key: string, value: string) {
      store[key] = String(value);
    },
    removeItem(key: string) {
      delete store[key];
    },
    clear() {
      Object.keys(store).forEach((k) => delete store[k]);
    },
  };
  globalThis.localStorage = mockStorage;
}
