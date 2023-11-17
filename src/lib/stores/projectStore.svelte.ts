import type { ProjectModel } from '$lib/models/ProjectModel.svelte';

export function createProjectStore() {
	let projects = $state<ProjectModel[]>([]);

	return {
		get value() {
			return projects;
		},
		set value(newProjects) {
			projects = newProjects;
		},
		push(projectModel: ProjectModel) {
			projects = [projectModel, ...projects];
		}
	};
}
