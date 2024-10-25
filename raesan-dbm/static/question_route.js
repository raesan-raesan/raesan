// handle create_question_form submition
const handleCreateQuestionFormSubmit = () => {
  let create_question_form = document.getElementById("create_question_form");
  if (
    (create_question_form.elements["body"].value.trim() === "" &&
      create_question_form.elements["body"].value.trim().length === 0) ||
    (create_question_form.elements["chapter_name"].value.trim() === "" &&
      create_question_form.elements["chapter_name"].value.trim().length ===
        0) ||
    (create_question_form.elements["subject_name"].value.trim() === "" &&
      create_question_form.elements["subject_name"].value.trim().length ===
        0) ||
    (create_question_form.elements["class_name"].value.trim() === "" &&
      create_question_form.elements["class_name"].value.trim().length === 0) ||
    (create_question_form.elements["chapter_id"].value.trim() === "" &&
      create_question_form.elements["chapter_id"].value.trim().length === 0)
  ) {
    alert("You cannot leave things empty!");
  } else {
    fetch("/api/question", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "",
        body: create_question_form.elements["body"].value,
        chapter_name: create_question_form.elements["chapter_name"].value,
        subject_name: create_question_form.elements["subject_name"].value,
        class_name: parseInt(create_question_form.elements["class_name"].value),
        chapter_id: create_question_form.elements["chapter_id"].value,
      }),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        if (document.getElementById("create_question_modal")) {
          document.getElementById("create_question_modal").close();
        }
      });
  }
};

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
      .then((data) => {
        if (document.getElementById("create_question_from_json_modal")) {
          document.getElementById("create_question_from_json_modal").close();
        }
      });
  }
};

// delete question handler
const deleteQuestion = (question_id, question_body) => {
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
      .then((data) => {
        document.getElementById(question_id).remove();
      });
  }
};

// edit question handler
const editQuestion = (question_id) => {
  let question = question_list.find((q) => q.id == question_id);
  if (question) {
    let question_row = document.getElementById(question.id);
    if (question_row) {
      question_row.innerHTML = `
			<td class="whitespace-nowrap">${question.id}</td>
			<td id="body" class="whitespace-nowrap"><input type="text" placeholder="Body" value="${question.body}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<td id="chapter_name" class="whitespace-nowrap"><input type="text" placeholder="Chapter" value="${question.chapter_name}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<td id="subject_name" class="whitespace-nowrap"><input type="text" placeholder="Subject" value="${question.subject_name}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<td id="class_name" class="whitespace-nowrap"><input type="number" placeholder="Class" value="${question.class_name}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<th>
				<div class="join">
				  <button
					class="btn btn-sm btn-outline btn-successfull join-item"
					onclick="updateQuestion(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(question))}')))"
				  >
				 Save
				  </button>
				  <button
					class="btn btn-sm btn-outline btn-error join-item"
					onclick="resetQuestion(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(question))}')))"
				  >
				  Reset
				  </button>
				</div>
			</th>
			`;
    }
  } else {
    alert("Something went wrong!");
  }
};

// update question handler
const updateQuestion = (question) => {
  const question_row = document.getElementById(question.id);
  let new_question = {
    id: question.id,
    body: question_row.querySelector("#body input").value,
    chapter_id: "",
    chapter_name: question_row.querySelector("#chapter_name input").value,
    subject_name: question_row.querySelector("#subject_name input").value,
    class_name: parseInt(question_row.querySelector("#class_name input").value),
  };
  question.chapter_id = "";
  // use `loadash` to compare structs
  if (_.isEqual(new_question, question)) {
    resetQuestion(question);
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
        const index = question_list.findIndex((q) => q.id === data.id);
        if (index !== -1) {
          question_list[index] = { ...data };
        } else {
          alert("Something went Terribly Wrong!");
        }
        resetQuestion(data);
      })
      .catch((err) => {
        resetQuestion(question);
        alert("Failed to update the Question");
        throw new Error(`HTTP error! Status: ${res.status}`);
      });
  }
};

// reset question handler
const resetQuestion = (question) => {
  document.getElementById(question.id).innerHTML = `
		<td>${question.id}</td>
		<td>${question.body}</td>
		<td>${question.chapter_name}</td>
		<td>${question.subject_name}</td>
		<td>${question.class_name}</td>
		<th>
			<div class="join">
			  <button
				class="btn btn-sm btn-outline btn-secondary join-item"
				onclick="editQuestion('${question.id}')"
			  >
			  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
			  </button>
			  <button
				class="btn btn-sm btn-outline btn-accent join-item"
				onclick="deleteQuestion('${question.id}','${question.body}')"
			  >
			  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
			  </button>
			</div>
		</th>
	`;
};

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
        question_table_body.innerHTML += `
					<tr id="${element.id}">
						<td>${element.id}</td>
						<td>${element.body}</td>
						<td>${element.chapter_name}</td>
						<td>${element.subject_name}</td>
						<td>${question.class_name}</td>
						<th>
							<div class="join">
							  <button
								class="btn btn-sm btn-outline btn-secondary join-item"
								onclick="editQuestion('${element.id}')"
							  >
							  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
							  </button>
							  <button
								class="btn btn-sm btn-outline btn-accent join-item"
								onclick="deleteQuestion('${element.id}','${element.name}')"
							  >
							  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
							  </button>
							</div>
						</th>
					</tr>
				`;
      });

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
