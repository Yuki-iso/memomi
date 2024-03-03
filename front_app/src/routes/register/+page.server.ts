import { redirect, fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
  default: async ({request}) => {
    const form = await request.formData(); 

    const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/create_user`, {
        method: "POST",
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams(form).toString(),
      });

    if (res.ok) {
      throw redirect(303, '/setup');
    }
    return fail(401);
  }
}
