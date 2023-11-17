import type { EProject } from '$lib/types/Project';

export class ProjectModel {
	id: string = '';
	name = $state('');
	directory = $state('');

	constructor(project: EProject) {
		this.id = project.id;
		this.name = project.name;
		this.directory = project.directory;
	}
}
