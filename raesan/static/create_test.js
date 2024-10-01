// main input
let create_test_input = {
  curr_step: 1,
  classes: [],
  subjects: [],
  chapters: [],
  format: {
    total_questions: 0,
  },
};

// handler for dispatching `updateStepEvent`
function dispatchUpdateStepEvent(next) {
  let updateStepEvent = new CustomEvent("updateStepEvent", {
    detail: {
      next: next,
    },
  });
  document.dispatchEvent(updateStepEvent);
}

// updating the form control buttons i.e `Next`, `Previous` buttons
function updateControlButtons() {
  if (create_test_input.curr_step != 1) {
    document.getElementById("prev_button").classList.remove("hidden");
  } else {
    document.getElementById("prev_button").classList.add("hidden");
  }

  if (create_test_input.curr_step === 5) {
    document.getElementById("next_button").innerHTML = "Create Test";
  } else {
    document.getElementById("next_button").innerHTML = "Next";
  }
}

// handler to update the `Stepper` component
function updateStepper() {
  let step_list = Array.from(document.getElementById("step_list").children);
  for (let i = 0; i < step_list.length; i++) {
    if (i === create_test_input.curr_step - 1) {
      step_list[i].classList.add("step-accent");
    } else {
      step_list[i].classList.remove("step-accent");
    }
  }
}

// handle displaying current step inputs
function handleStepInputDisplayUpdate() {
  let input_list = Array.from(
    document.getElementById("create_test_input_box").children,
  );
  for (let i = 0; i < input_list.length; i++) {
    if (i === create_test_input.curr_step - 1) {
      input_list[i].classList.remove("hidden");
    } else {
      input_list[i].classList.add("hidden");
    }
  }
}

// load data into `create_test_input` from DOM
function loadCreateTestInputData() {
  if (create_test_input.curr_step == 2) {
    create_test_input.classes = Array.from(
      document.getElementById("class_input_form").children,
    )
      .filter((element) => {
        return element.children[1].checked;
      })
      .map((element) => {
        return element.id;
      });
  }
  if (create_test_input.curr_step == 3) {
    create_test_input.subjects = Array.from(
      document.getElementById("subject_input_form").children,
    )
      .filter((element) => {
        return element.children[1].checked;
      })
      .map((element) => {
        return element.id;
      });
  }
  if (create_test_input.curr_step == 4) {
    create_test_input.chapters = Array.from(
      document.getElementById("chapter_input_form").children,
    )
      .filter((element) => {
        return element.children[1].checked;
      })
      .map((element) => {
        return element.id;
      });
  }
  if (create_test_input.curr_step == 5) {
    create_test_input.format.total_questions = parseInt(
      document.getElementById("format_input_form").children[1].value,
    );
  }
}

// listener for `updateStepEvent`
document.addEventListener("updateStepEvent", function (event) {
  if (event.detail.next === true) {
    if (create_test_input.curr_step === 5) {
      // POST request to the server
      fetch("/api/create-test", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(create_test_input),
      })
        .then((response) => {
          if (response.redirected) {
            window.location.href = response.url;
          } else {
            console.log(response);
          }
        })
        .cathc((error) => {
          console.error(
            "Failed to make a request to the Server, Error:",
            error,
          );
        });
      return;
    } else {
      create_test_input.curr_step += 1;
    }
  }
  if (event.detail.next === false) {
    if (create_test_input.curr_step === 1) {
      alert("No Can Do!");
      return;
    } else {
      create_test_input.curr_step -= 1;
    }
  }

  if (event.detail.next === true) {
    loadCreateTestInputData();
  }
  updateControlButtons();
  updateStepper();
  handleStepInputDisplayUpdate();

  if (create_test_input.curr_step === 2 && event.detail.next === true) {
    let subject_input_form = document.getElementById("subject_input_form");
    subject_input_form.innerHTML = "";
    create_test_input.classes.forEach((class_id) => {
      dataset.subjects
        .filter((dataset_subject) => {
          return dataset_subject.class_id === class_id;
        })
        .forEach((subject) => {
          subject_input_form.innerHTML += `
					<label id="${subject.id}" class="label cursor-pointer gap-[15px] border border-gray-500 rounded-[6px] px-4 py-3 max-w-[220px] w-full">
						<span class="label-text">${subject.name}</span>
						<input name=${subject.name} type="checkbox" class="checkbox" />
					</label>
				`;
        });
    });
  }

  if (create_test_input.curr_step === 3 && event.detail.next === true) {
    let chapter_input_form = document.getElementById("chapter_input_form");
    chapter_input_form.innerHTML = "";
    create_test_input.subjects.forEach((subject_id) => {
      dataset.chapters
        .filter((dataset_chapter) => {
          return dataset_chapter.subject_id === subject_id;
        })
        .forEach((chapter) => {
          chapter_input_form.innerHTML += `
					<label id="${chapter.id}" class="label cursor-pointer gap-[15px] border border-gray-500 rounded-[6px] px-4 py-3 max-w-[220px] w-full">
						<span class="label-text">${chapter.name}</span>
						<input name=${chapter.name} type="checkbox" class="checkbox" />
					</label>
				`;
        });
    });
  }

  if (create_test_input.curr_step == 5 && event.detail.next === true) {
    let final_input_display = document.getElementById("final_input_display");
    final_input_display.innerHTML = "";
    final_input_display.innerHTML += `
		<div class="join join-vertical bg-base-200">
		  <div class="flex gap-[5px] bg-base-200 join-item justify-center pt-2">
			  <p class="font-semibold">Total Questions: </p>
			  <p class="underline decoration-accent underline-offset-4">${create_test_input.format.total_questions}</p>
		  </div>
		  <div class="collapse collapse-arrow bg-base-200 max-w-[250px] join-item">
			  <input type="checkbox" />
			  <div class="collapse-title text-xl font-medium">Selected Chapters</div>
			  <ul id="final_chapters_list" class="flex flex-col items-left collapse-content list-disc ml-[15px]">
			  </ul>
		  </div>
		</div>
	  `;
    let final_chapters_list = document.getElementById("final_chapters_list");
    create_test_input.chapters
      .map((chapter_id) => {
        return dataset.chapters.find((dataset_chapter) => {
          return dataset_chapter.id === chapter_id;
        });
      })
      .forEach((chapter) => {
        final_chapters_list.innerHTML += `
		  	<li class="">${chapter.name}</li>
		  `;
      });
  }
});
