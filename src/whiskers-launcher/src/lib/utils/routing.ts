import { goto } from '$app/navigation';

export function goToRoute(route: string) {
	goto(route, { replaceState: true });
}