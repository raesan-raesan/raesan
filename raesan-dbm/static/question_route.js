// convert integer to roman number
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
window.integerToRoman = integerToRoman;

// split the latex string for easy rendering
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
window.splitLatexBodyString = splitLatexBodyString;

// convert unix to readable
const unix_to_readable = (unix_time) => {
  const date = new Date(unix_time * 1000);
  return date.toLocaleDateString() + " " + date.toLocaleTimeString();
};
window.unix_to_readable = unix_to_readable;

// update unix time stamps of the whole web page
const updateUnixTimeStamps = () => {
  document.querySelectorAll("td[data-timestamp]").forEach((element) => {
    const unix_time = element.getAttribute("data-timestamp");
    element.textContent = window.unix_to_readable(unix_time);
  });
};
window.updateUnixTimeStamps = updateUnixTimeStamps;
updateUnixTimeStamps(); // run at the beginning

// render questions body
const renderQuestionBody = (question_body, question_element) => {
  question_element.innerHTML = `
		<div class="w-full flex flex-col gap-[20px]" id="body">
			<div id="text-body"></div>
			<div class="overflow-auto w-full max-w-[260px] sm:max-w-[350px] md:max-w-[450px] lg:max-w-[1000px] flex items-center justify-left" id="latex-body"></div>
		</div>
		`;

  const parts = window.splitLatexBodyString(question_body);
  let text_body = "";
  let latex_body = "";
  let latex_count = 0;

  for (let j = 0; j < parts.length; j++) {
    if (j % 2 === 1) {
      latex_count += 1;
      let curr_num = window.integerToRoman(latex_count);
      latex_body += `(${curr_num})\\space\\space ${parts[j]} \\quad`;
      text_body += `<b>(${curr_num})</b>`;
    } else {
      text_body += parts[j];
    }
  }
  question_element
    .querySelector("#body")
    .querySelector("#text-body").innerHTML = text_body;

  if (latex_body.trim().length == 0) {
    question_element
      .querySelector("#body")
      .querySelector("#latex-body")
      .remove();
  } else {
    katex.render(
      latex_body,
      question_element.querySelector("#body").querySelector("#latex-body"),
      {
        throwOnError: false,
        displayMode: true,
      },
    );
  }
};
window.renderQuestionBody = renderQuestionBody;

window.chapter_list.forEach((element) => {
  document.getElementById("create_question_form").elements[
    "chapter_display_name"
  ].innerHTML +=
    `<option value="${element.display_name}">${element.display_name}</option>`;
});
// handle create_question_form submition
const handleCreateQuestionFormSubmit = () => {
  let create_question_form = document.getElementById("create_question_form");
  if (
    create_question_form.elements["body"].value.trim() === "" ||
    create_question_form.elements["body"].value.trim().length === 0 ||
    create_question_form.elements["chapter_display_name"].value.trim() === "" ||
    create_question_form.elements["chapter_display_name"].value.trim()
      .length === 0 ||
    create_question_form.elements["chapter_display_name"].value === "0"
  ) {
    alert("You cannot leave things empty!");
  } else {
    let curr_chapter = window.chapter_list.find(
      (ch) =>
        ch.display_name ==
        create_question_form.elements["chapter_display_name"].value,
    );
    fetch("/api/question", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "",
        body: create_question_form.elements["body"].value,
        chapter_name: curr_chapter.name,
        subject_name: curr_chapter.subject_name,
        class_name: curr_chapter.class_name,
        chapter_id: curr_chapter.id,
        created_at: 0,
        updated_at: 0,
      }),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((_) => {
        if (document.getElementById("create_question_modal")) {
          document.getElementById("create_question_modal").close();
        }
      });
  }
};
window.handleCreateQuestionFormSubmit = handleCreateQuestionFormSubmit;

// handle create_question_from_json_input submition
document.getElementById("create_question_from_json_input").value = "";
const handleCreateQuestionFromJsonFormSubmit = () => {
  let create_question_from_json_input = document.getElementById(
    "create_question_from_json_input",
  );
  if (
    create_question_from_json_input.value.trim() === "" &&
    create_question_from_json_input.value.trim().length === 0
  ) {
    alert("Atleast enter something!");
  } else {
    fetch("/api/question/json", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(
        JSON.parse(create_question_from_json_input.value),
      ).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((_) => {
        if (document.getElementById("create_question_from_json_modal")) {
          document.getElementById("create_question_from_json_modal").close();
        }
      });
  }
};
window.handleCreateQuestionFromJsonFormSubmit =
  handleCreateQuestionFromJsonFormSubmit;

// delete question handler
const handleDeleteQuestion = (question_id, question_body) => {
  let choice = confirm(`WARNING! Do you want to delete '${question_body}'`);
  if (choice == true) {
    fetch(`/api/question/${question_id}`, {
      method: "DELETE",
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.text();
      })
      .then((_) => {
        document.getElementById(question_id).remove();
      });
  }
};
window.handleDeleteQuestion = handleDeleteQuestion;

// edit question handler
const handleEditQuestion = (question_id) => {
  let question = window.question_list.find((q) => q.id == question_id);
  if (question) {
    let question_row = document.getElementById(question.id);
    if (question_row) {
      question_row.innerHTML = `
			<td class="whitespace-nowrap">${question.id}</td>
			<td id="body" class="whitespace-nowrap"><input type="text" placeholder="Body" value="${question.body}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<td id="chapter_display_name" class="whitespace-nowrap"><select id="chapter" class="select select-bordered w-full max-w-xs"></select></td>
			<th>
				<div class="join">
				  <button
					class="btn btn-sm btn-outline btn-successfull join-item"
					onclick="handleUpdateQuestion(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(question))}')))"
				  >
				 Save
				  </button>
				  <button
					class="btn btn-sm btn-outline btn-error join-item"
					onclick="handleResetQuestion(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(question))}')))"
				  >
				  Reset
				  </button>
				</div>
			</th>
			`;
      window.chapter_list.forEach((element) => {
        question_row.querySelector("#chapter_display_name select").innerHTML +=
          `<option ${element.display_name == `${question.class_name} - ${question.subject_name} - ${question.chapter_name}` ? "selected" : ""} value="${element.display_name}">${element.display_name}</option>`;
      });
    }
  } else {
    alert("Something went wrong!");
  }
};
window.handleEditQuestion = handleEditQuestion;

// update question handler
const handleUpdateQuestion = (question) => {
  const question_row = document.getElementById(question.id);
  let curr_chapter = window.chapter_list.find(
    (ch) =>
      ch.display_name ==
      question_row.querySelector("#chapter_display_name select").value,
  );
  let new_question = {
    id: question.id,
    body: question_row.querySelector("#body input").value,
    chapter_id: curr_chapter.id,
    chapter_name: curr_chapter.name,
    subject_name: curr_chapter.subject_name,
    class_name: curr_chapter.class_name,
    created_at: question.created_at,
    updated_at: question.updated_at,
  };
  // use `loadash` to compare structs
  if (_.isEqual(new_question, question)) {
    handleResetQuestion(question);
  } else {
    fetch("/api/question", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(new_question).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        // update the question in the question_list
        const index = window.question_list.findIndex((q) => q.id === data.id);
        if (index !== -1) {
          window.question_list[index] = { ...data };
        } else {
          alert("Something went Terribly Wrong!");
        }
        handleResetQuestion(data);
      })
      .catch((_) => {
        handleResetQuestion(question);
        alert("Failed to update the Question");
        throw new Error(`HTTP error! Status: ${res.status}`);
      });
  }
};
window.handleUpdatQuestion = handleUpdateQuestion;

// reset question handler
const handleResetQuestion = (question) => {
  document.getElementById(question.id).innerHTML = `
		<td>${question.id}</td>
		<td id="question-body"></td>
		<td>${question.chapter_name}</td>
		<td>${question.subject_name}</td>
		<td>${question.class_name}</td>
		<td data-timestamp="${question.created_at}"></td>
		<td data-timestamp="${question.updated_at}"></td>
		<th>
			<div class="join">
			  <button
				class="btn btn-sm btn-outline btn-secondary join-item"
				onclick="handleEditQuestion('${question.id}')"
			  >
			  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
			  </button>
			  <button
				class="btn btn-sm btn-outline btn-accent join-item"
				onclick="handleDeleteQuestion('${question.id}','${question.body}')"
			  >
			  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
			  </button>
			</div>
		</th>
	`;
  updateUnixTimeStamps();

  window.renderQuestionBody(
    question.body,
    document.getElementById(question.id).querySelector("#question-body"),
  );
};
window.handleResetQuestion = handleResetQuestion;

let curr_page = 1;

// Function to fetch and append new data
function fetchAndAppendData() {
  let question_table_body = document.getElementById("question_table_body");
  curr_page += 1;

  fetch(`/api/question?page=${curr_page}`, { method: "GET" })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      if (data.length === 0) {
        // If no more data, stop observing
        observer.disconnect();
        return;
      }

      // Append the new data to the table body
      data.forEach((element) => {
        window.question_list.push(element); // push the element to the question list
        question_table_body.innerHTML += `
					<tr id="${element.id}">
						<td>${element.id}</td>
						<td id="question-body"></td>
						<td>${element.chapter_name}</td>
						<td>${element.subject_name}</td>
						<td>${element.class_name}</td>
						<td data-timestamp="${element.created_at}"></td>
						<td data-timestamp="${element.updated_at}"></td>
						<th>
							<div class="join">
							  <button
								class="btn btn-sm btn-outline btn-secondary join-item"
								onclick="handleEditQuestion('${element.id}')"
							  >
							  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
							  </button>
							  <button
								class="btn btn-sm btn-outline btn-accent join-item"
								onclick="handleDeleteQuestion('${element.id}','${element.name}')"
							  >
							  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
							  </button>
							</div>
						</th>
					</tr>
				`;
        window.renderQuestionBody(
          element.body,
          document.getElementById(element.id).querySelector("#question-body"),
        );
      });
      updateUnixTimeStamps();

      // Update the observer to observe the new last element
      const newLastElement = question_table_body.lastElementChild;
      if (newLastElement) {
        observer.observe(newLastElement);
      }
    })
    .catch((err) => {
      console.error("Failed to fetch data:", err);
    });
}

// IntersectionObserver to load more data when the last element is visible
const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        // Stop observing the current element
        observer.unobserve(entry.target);
        // Fetch and append new data
        fetchAndAppendData();
      }
    });
  },
  {
    threshold: 0.1, // when at least 10% of the element is visible
  },
);

// Start observing when the page loads
document.addEventListener("DOMContentLoaded", () => {
  const initialLastElement = document.getElementById(
    "question_table_body",
  ).lastElementChild;
  if (initialLastElement) {
    observer.observe(initialLastElement);
  }
});
