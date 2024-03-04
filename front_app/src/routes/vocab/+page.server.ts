export async function load() {
	/*const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/new_words/65cb371d3390aaa918373c02`, {
    method: "GET",
    headers: {
      'Content-Type': 'application/json',
    },
  }*/

	//const data = await res.json();
	const cards = [
		{
			kanji: '〜か月',
			jp_sentence: '日本[にほん]に 来[き]ました。',
			english: '3 months ago',
			en_sentence: 'I2 came to Japan',
			status: 'New'
		},
		{
			kanji: '〜か月2',
			jp_sentence: '日本[にほん]に 来[き]ました。',
			english: '2 months ago',
			en_sentence: 'I came to Japan',
			status: 'Review'
		},
		{
			kanji: '〜か月3',
			jp_sentence: '日本[にほん]に 来[き]ました。',
			english: '1 month ago',
			en_sentence: 'I came to Japan',
			status: 'New'
		}
	];
	return {
		props: cards
	};
}

export async function update(id: number, status: string){
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/update_status`, {
    method: "PUT",
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({id: id, status: status}),
  });
  return res.json();
}
