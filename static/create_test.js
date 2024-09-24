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
      window.location.href = `/test?create_test_input=${encodeURIComponent(JSON.stringify(create_test_input))}`;
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
    final_input_display.innerHTML += `
		  <p>Total Questions: ${create_test_input.format.total_questions}</p>	
		  <p class="font-semibold text-xl">Selected Chapters</p>
	  `;
    create_test_input.chapters
      .map((chapter_id) => {
        return dataset.chapters.find((dataset_chapter) => {
          return dataset_chapter.id === chapter_id;
        });
      })
      .forEach((chapter) => {
        final_input_display.innerHTML += `
		  	<p class="">${chapter.name}</p>
		  `;
      });
  }
});
