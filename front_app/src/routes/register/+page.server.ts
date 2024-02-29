import { redirect, fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

//const backend = import.meta.env.BACKEND_URL;

export const actions: Actions = {
  default: async ({request}) => {
    const form = await request.formData(); 

    const res = await fetch("http://localhost:8080/create_user",
      {
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
