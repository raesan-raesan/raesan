// main input and control variable
let loading = false;
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

  if (create_test_input.curr_step == 5) {
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
  create_test_input.classes = Array.from(
    new FormData(document.getElementById("class_input_form")).entries(),
  ).map((element) => {
    return parseInt(element[0]);
  });
}

// fetch data into `create_test_input` from server
async function fetchCreateTestInputData() {
  try {
    loading = true;
    console.log("loading...");
    const res = await fetch("/api/create-test", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(create_test_input),
    });
    create_test_input = await res.json();
  } catch (error) {
    console.log("Failed to fetch data from the server, Error:", error);
  } finally {
    loading = false;
    console.log("loaded!");
  }
}

// listener for `updateStepEvent`
document.addEventListener("updateStepEvent", function (event) {
  if (event.detail.next == true) {
    if (create_test_input.curr_step === 5) {
      alert("No Can Do!");
      return;
    } else {
      create_test_input.curr_step += 1;
    }
  }
  if (event.detail.next == false) {
    if (create_test_input.curr_step === 1) {
      alert("No Can Do!");
      return;
    } else {
      create_test_input.curr_step -= 1;
    }
  }

  if (create_test_input.curr_step < 4 && event.detail.next == true) {
    loadCreateTestInputData();
    fetchCreateTestInputData();
  }
  if (create_test_input.curr_step >= 4 && event.detail.next == true) {
    loadCreateTestInputData();
  }

  updateControlButtons();
  updateStepper();
  handleStepInputDisplayUpdate();

  if (create_test_input.curr_step == 2) {
    let subject_input_form = document.getElementById("subject_input_form");
    if (Array.from(subject_input_form.children).length === 0) {
      create_test_input.subjects.forEach((subject) => {
        subject_input_form.innerHTML += `
					<label class="label cursor-pointer gap-[15px] border border-gray-500 rounded-[6px] px-4 py-3 max-w-[220px] w-full">
						<span class="label-text">${subject}</span>
						<input type="checkbox" class="checkbox" />
					</label>
				`;
      });
    }
  }
  if (create_test_input.curr_step == 3) {
    let chapter_input_form = document.getElementById("chapter_input_form");
    if (Array.from(chapter_input_form.children).length === 0) {
      create_test_input.chapters.forEach((chapter) => {
        chapter_input_form.innerHTML += `
					<label class="label cursor-pointer gap-[15px] border border-gray-500 rounded-[6px] px-4 py-3 max-w-[220px] w-full">
						<span class="label-text">${chapter.name}</span>
						<input type="checkbox" class="checkbox" />
					</label>
				`;
      });
    }
  }
});
