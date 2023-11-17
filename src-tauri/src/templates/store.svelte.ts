export const createNAME_STOREStore = () => {
	let data = $state();

	return {
		set: (value: string) => {
			data = value;
		},
		get: () => {
			return data;
		}
	};
};

export const EXPORT_NAME_STORE = createNAME_STOREStore();