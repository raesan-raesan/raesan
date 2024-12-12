function integerToRoman(num) {
  const values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
  const symbols = [
    "m",
    "cm",
    "d",
    "cd",
    "c",
    "xc",
    "l",
    "xl",
    "x",
    "ix",
    "v",
    "iv",
    "i",
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
function splitLatexBodyString(input) {
  const result = [];
  let current = "";
  let inDelimiters = false;
  let i = 0;

  while (i < input.length) {
    if (!inDelimiters) {
      // Look for the opening $$
      if (input[i] === "$" && input[i + 1] === "$") {
        if (current) result.push(current); // Push the text outside $$ to result
        current = "";
        inDelimiters = true; // Entering $$ block
        i += 2; // Skip the $$
      } else {
        current += input[i];
        i++;
      }
    } else {
      // Look for the closing $$
      if (input[i] === "$" && input[i + 1] === "$") {
        result.push(current); // Push the text inside $$ to result
        current = "";
        inDelimiters = false; // Exiting $$ block
        i += 2; // Skip the $$
      } else {
        current += input[i];
        i++;
      }
    }
  }

  // Push any remaining text outside $$ after the loop
  if (current) result.push(current);

  return result;
}
window.renderQuestions = (questions) => {
  const questions_list = document.getElementById("questions_list");
  let i = 0;
  while (i < questions.length) {
    questions_list.innerHTML += `
				<div class="flex items-top justify-start gap-[20px] w-full" id="${questions[i].id}">
					<p class="text-xl tooltip tooltip-right" data-tip="${questions[i].id}">(${i + 1})</p> 
					<div class="w-full flex flex-col gap-[20px]" id="body">
						<div id="text-body"></div>
						<div class="overflow-auto w-full max-w-[260px] sm:max-w-[350px] md:max-w-[450px] lg:max-w-[1000px] flex items-center justify-left" id="latex-body"></div>
					</div>
				</div>
			`;

    const parts = splitLatexBodyString(questions[i].body);
    let text_body = "";
    let latex_body = "";
    let latex_count = 0;

    for (let j = 0; j < parts.length; j++) {
      if (j % 2 === 1) {
        latex_count += 1;
        let curr_num = integerToRoman(latex_count);
        latex_body += `(${curr_num})\\space\\space ${parts[j]} \\quad`;
        text_body += `<b>(${curr_num})</b>`;
      } else {
        text_body += parts[j];
      }
    }
    document
      .getElementById(questions[i].id)
      .querySelector("#body")
      .querySelector("#text-body").innerHTML = text_body;

    if (latex_body.trim().length == 0) {
      document
        .getElementById(questions[i].id)
        .querySelector("#body")
        .querySelector("#latex-body")
        .remove();
    } else {
      katex.render(
        latex_body,
        document
          .getElementById(questions[i].id)
          .querySelector("#body")
          .querySelector("#latex-body"),
        {
          throwOnError: false,
          displayMode: true,
        },
      );
    }
    i++;
  }
};
