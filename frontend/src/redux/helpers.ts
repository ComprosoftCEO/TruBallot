import { initialState, RootState } from 'redux/state';
import { useSelector } from 'react-redux';
import { store } from 'redux/store';
import { updateNestedState, dynamicUpdateNestedState, replaceNestedState, dynamicReplaceNestedState } from './actions';

/// Can either set from a plain JavaScript object, function, or promise
export type MergeOrActionSource<T> = Partial<T> | ((input: T) => Partial<T>) | Promise<Partial<T>>;
export type MergeFunction<T> = (input: MergeOrActionSource<T>) => void;

/// Can either set from a plain JavaScript object, function, or promise
export type SetOrActionSource<T> = T | ((input: T) => T) | Promise<T>;
export type SetFunction<T> = (iput: SetOrActionSource<T>) => void;

/// Special type for properties
export type PropertyOrActionSource<Prop, State> = Prop | ((input: State) => Prop) | Promise<Prop>;
export type PropertyFunction<Props, State> = (input: PropertyOrActionSource<Props, State>) => void;

/**
 * Higher-order hook for selecting from a nested state inside the root state
 *
 * @param nestedState Key of the nested state
 * @returns Hook function
 */
export const nestedSelectorHook =
  <NestedState extends keyof RootState>(nestedState: NestedState) =>
  <T>(selector: (input: RootState[NestedState]) => T, equalityFn?: (left: T, right: T) => boolean): T =>
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useSelector<RootState, T>((state) => selector(state[nestedState]), equalityFn);

/**
 * Higher-order function that serves as a "getState()" method for a nested state
 *
 * @param nestedState Key of the nested state
 * @returns Nested state
 */
export const getNestedState =
  <NestedState extends keyof RootState>(nestedState: NestedState) =>
  (): RootState[NestedState] =>
    store.getState()[nestedState];

/**
 * Higher-order function to merge two states together
 *   Input is a partial data type
 *
 * The input can either be a PARTIAL value, a function that returns a PARTIAL value,
 *  or a promise that returns a PARTIAL value
 *
 * Two call signatures: mergeNestedState("") --> Returns higher-order function
 *                      mergeNestedState("", data) ---> Applies data to the nested state
 *
 * @param nestedState Key of the nested state
 * @param data If set, implicitly applies without returning the higher-order functtion
 * @param input New value to set
 * @returns Higher-order function to merge the nested state
 */
export function mergeNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
): MergeFunction<RootState[NestedState]>;

export function mergeNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
  data: MergeOrActionSource<RootState[NestedState]>,
): void;

export function mergeNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
  data?: MergeOrActionSource<RootState[NestedState]>,
) {
  const mergeFunction = (input: MergeOrActionSource<RootState[NestedState]>): void => {
    // Is it a function?
    if (typeof input === 'function') {
      store.dispatch(dynamicUpdateNestedState(nestedState, input) as any);
      return;
    }

    // Is it a promise?
    if (typeof input === 'object' && typeof (input as Promise<any>)?.then === 'function') {
      (input as Promise<Partial<RootState[NestedState]>>).then((output) =>
        store.dispatch(updateNestedState(nestedState, output)),
      );
      return;
    }

    // Okay, it is a plain JavaScript object
    store.dispatch(updateNestedState(nestedState, input as Partial<RootState[NestedState]>));
  };

  // Apply the function if the data is provided
  if (data !== undefined) {
    return mergeFunction(data);
  }
  return mergeFunction;
}

/**
 * Higher-order function to set all values in the state
 *   Input is a complete data type
 *
 * The input can either be a FULL value, a function that returns a FULL value,
 *  or a promise that returns a FULL value
 *
 * Two call signatures: setNestedState("") --> Returns higher-order function
 *                      setNestedState("", data) ---> Applies data to the nested state
 *
 * @param nestedState Key of the nested state
 * @param data If set, implicitly applies without returning the higher-order functtion
 * @param input New value to set.
 * @returns Higher-order function to merge the nested state
 */
export function setNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
): SetFunction<RootState[NestedState]>;

export function setNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
  data: SetOrActionSource<RootState[NestedState]>,
): void;

export function setNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
  data?: SetOrActionSource<RootState[NestedState]>,
) {
  const setFunction = (input: SetOrActionSource<RootState[NestedState]>) => {
    // Is it a function?
    if (typeof input === 'function') {
      store.dispatch(dynamicReplaceNestedState(nestedState, input) as any);
      return;
    }

    // Is it a promise?
    if (typeof input === 'object' && typeof (input as Promise<any>)?.then === 'function') {
      (input as Promise<RootState[NestedState]>).then((output) =>
        store.dispatch(replaceNestedState(nestedState, output)),
      );
      return;
    }

    // Okay, it is a plain JavaScript object
    store.dispatch(replaceNestedState(nestedState, input as RootState[NestedState]));
  };

  // Apply the function if the data is provided
  if (data !== undefined) {
    return setFunction(data);
  }
  return setFunction;
}

/**
 * Higher-order function to set specific properties inside a nested state
 *
 * For example:
 *   const setProperty = setNestedProperty("login");
 *   <TextInput onChange={setProperty("username")} />
 *
 * This function accepts either a value, closure, or promise
 * IF THE INPUT IS A FUNCTION, it will try to call it, even
 * if you want to store that in the store
 *
 * @param nestedState Nested state of the global store
 * @param property Name of the property
 * @param newValue New value for the property
 * @returns Higher-order function
 */
export const setNestedProperty =
  <NestedState extends keyof RootState>(nestedState: NestedState) =>
  (property: keyof RootState[NestedState]) =>
    setProperty(nestedState, property);

function setProperty<NestedState extends keyof RootState, Property extends keyof RootState[NestedState]>(
  nestedState: NestedState,
  property: Property,
): PropertyFunction<RootState[NestedState][Property], RootState[NestedState]> {
  return (newValue: PropertyOrActionSource<RootState[NestedState][Property], RootState[NestedState]>) => {
    // Is it a function?
    if (typeof newValue === 'function') {
      store.dispatch(
        dynamicUpdateNestedState(nestedState, (state) => ({ [property]: (newValue as any)(state) } as any)) as any,
      );
      return;
    }

    // Is it a promise?
    if (typeof newValue === 'object' && typeof (newValue as Promise<any>)?.then === 'function') {
      (newValue as Promise<RootState[NestedState][Property]>).then((output) =>
        store.dispatch(updateNestedState(nestedState, { [property]: output } as any)),
      );
      return;
    }

    // Okay, it is a plain JavaScript object
    store.dispatch(updateNestedState(nestedState, { [property]: newValue as RootState[NestedState][Property] } as any));
  };
}

/**
 * Helpful function to clear the nested state
 *
 * @param nestedState Nested state to clear
 * @param initialState Initial value of the state
 */
export function clearNestedState<NestedState extends keyof RootState>(
  nestedState: NestedState,
  initial: RootState[NestedState] = initialState[nestedState],
) {
  store.dispatch(updateNestedState(nestedState, initial));
}
