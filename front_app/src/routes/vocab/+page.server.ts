export async function load(){
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/new_words/65cb371d3390aaa918373c02`, {
    method: "GET",
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const data = await res.json();
  console.log(data);
  return {
    props: {
      data
    }
  }
}
