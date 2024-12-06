function integerToRoman(num) {
  const values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
  const symbols = [
    "M",
    "CM",
    "D",
    "CD",
    "C",
    "XC",
    "L",
    "XL",
    "X",
    "IX",
    "V",
    "Iv",
    "I",
  ];
  let roman = "";
  for (let i = 0; i < values.length; i++) {
    while (num >= values[i]) {
      roman += symbols[i];
      num -= values[i];
    }
  }
  return roman;
}
window.renderQuestions = (questions) => {
  const questions_list = document.getElementById("questions_list");
  let i = 0;
  while (i < questions.length) {
    questions_list.innerHTML += `
				<div class="flex items-top justify-start gap-[20px] w-full" id="${questions[i].id}">
					<p class="text-xl">(${i + 1})</p> 
					<div class="w-full flex flex-col gap-[20px]" id="body">
						<div id="text-body"></div>
						<div class="w-full flex justify-start gap-[10px] overflow-auto" id="latex-body"></div>
					</div>
				</div>
			`;

    const parts = questions[i].body.split(/\[\[(.*?)\]\]/g);
    let text_body = "";
    let latex_content = [];
    let latex_count = 0;

    for (let j = 0; j < parts.length; j++) {
      if (j % 2 === 1) {
        const placeholderContent = parts[j];
        latex_count += 1;
        let latex_object = {
          index: integerToRoman(latex_count),
          body: katex.renderToString(placeholderContent, {
            throwOnError: false,
            displayMode: true,
          }),
        };
        latex_content.push(latex_object);
        text_body += `<b>(${latex_object.index})</b>`;
      } else {
        text_body += parts[j];
      }
    }
    document
      .getElementById(questions[i].id)
      .querySelector("#body")
      .querySelector("#text-body").innerHTML = text_body;

    for (let k = 0; k < latex_content.length; k++) {
      let latex_element = document
        .getElementById(questions[i].id)
        .querySelector("#body")
        .querySelector("#latex-body");
      latex_element.innerHTML += `<div>(${latex_content[k].index})</div>`;
      latex_element.innerHTML += `<div class="mr-[20px]">${latex_content[k].body}</div>`;
    }
    i++;
  }
};
