export const useSelector = (initialState = '') => {
	let value = $state(initialState);

	return {
		get value() {
			return value;
		},
		set(v: any) {
			value = v;
		},
		isEqual(v: any) {
			return value === v;
		}
	};
};
