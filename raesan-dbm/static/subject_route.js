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
updateUnixTimeStamps(); // run at the beginning
window.updateUnixTimeStamps = updateUnixTimeStamps;

window.class_list.forEach((element) => {
  document.getElementById("create_subject_form").elements[
    "class_name"
  ].innerHTML += `<option value="${element.name}">${element.name}</option>`;
});
// handle create_subject_form submition
const handleCreateSubjectFormSubmit = () => {
  let create_subject_form = document.getElementById("create_subject_form");
  if (
    create_subject_form.elements["name"].value.trim() === "" ||
    create_subject_form.elements["name"].value.trim().length === 0 ||
    create_subject_form.elements["class_name"].value.trim() === "" ||
    create_subject_form.elements["class_name"].value.trim().length === 0 ||
    parseInt(create_subject_form.elements["class_name"].value) === 0
  ) {
    alert("You cannot leave things empty!");
  } else {
    fetch("/api/subject", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "",
        name: create_subject_form.elements["name"].value,
        display_name: `${create_subject_form.elements["class_name"].value} - ${create_subject_form.elements["name"].value}`,
        class_id: window.class_list.find(
          (cl) =>
            cl.name ==
            parseInt(create_subject_form.elements["class_name"].value),
        ).id,
        class_name: parseInt(create_subject_form.elements["class_name"].value),
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
      .then((data) => {
        if (document.getElementById("create_subject_modal")) {
          document.getElementById("create_subject_modal").close();
          document.getElementById("subject_table_body").innerHTML += `
				<tr id="${data.id}">
					<td>${data.id}</td>
					<td class="max-w-[250px]">${data.name}</td>
					<td>${data.class_name}</td>
					<td data-timestamp="${data.created_at}"></td>
					<td data-timestamp="${data.updated_at}"></td>
					<th>
						<div class="join">
						  <button
							class="btn btn-sm btn-outline btn-secondary join-item"
							onclick="handleEditSubject('${data.id}')"
						  >
						  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
						  </button>
						  <button
							class="btn btn-sm btn-outline btn-accent join-item"
							onclick="handleDeleteSubject('${data.id}','${data.name}')"
						  >
						  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
						  </button>
						</div>
					</th>
				</tr>
				`;
          updateUnixTimeStamps();
        }
      });
  }
};
window.handleCreateSubjectFormSubmit = handleCreateSubjectFormSubmit;

// handle create_subject_from_json_input submition
document.getElementById("create_subject_from_json_input").value = "";
const handleCreateSubjectFromJsonFormSubmit = () => {
  let create_subject_from_json_input = document.getElementById(
    "create_subject_from_json_input",
  );
  if (
    create_subject_from_json_input.value.trim() === "" &&
    create_subject_from_json_input.value.trim().length === 0
  ) {
    alert("Atleast enter something!");
  } else {
    fetch("/api/subject/json", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(
        JSON.parse(create_subject_from_json_input.value),
      ).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        if (document.getElementById("create_subject_from_json_modal")) {
          document.getElementById("create_subject_from_json_modal").close();
          data.forEach((element) => {
            document.getElementById("subject_table_body").innerHTML += `
					<tr id="${element.id}">
						<td>${element.id}</td>
						<td class="max-w-[250px]">${element.name}</td>
						<td>${element.class_name}</td>
						<td data-timestamp="${element.created_at}"></td>
						<td data-timestamp="${element.updated_at}"></td>
						<th>
							<div class="join">
							  <button
								class="btn btn-sm btn-outline btn-secondary join-item"
								onclick="handleEditSubject('${element.id}')"
							  >
							  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
							  </button>
							  <button
								class="btn btn-sm btn-outline btn-accent join-item"
								onclick="handleDeleteSubject('${element.id}','${element.name}')"
							  >
							  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
							  </button>
							</div>
						</th>
					</tr>
					`;
            updateUnixTimeStamps();
          });
        }
      });
  }
};
window.handleCreateSubjectFromJsonFormSubmit =
  handleCreateSubjectFromJsonFormSubmit;

// delete subject handler
const handleDeleteSubject = (subject_id, subject_name) => {
  let choice = confirm(`WARNING! Do you want to delete '${subject_name}'`);
  if (choice == true) {
    fetch(`/api/subject/${subject_id}`, {
      method: "DELETE",
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.text();
      })
      .then((_) => {
        document.getElementById(subject_id).remove();
      });
  }
};
window.handleDeleteSubject = handleDeleteSubject;

// edit subject handler
const handleEditSubject = (subject_id) => {
  let subject = window.subject_list.find((sb) => sb.id == subject_id);
  if (subject) {
    let subject_row = document.getElementById(subject.id);
    if (subject_row) {
      subject_row.innerHTML = `
			<td class="whitespace-nowrap">${subject.id}</td>
			<td id="name" class="whitespace-nowrap"><input type="text" placeholder="Name" value="${subject.name}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<td id="class_name" class="whitespace-nowrap"><select id="suj" class="select select-bordered w-full max-w-xs"></select></td>
			<th>
				<div class="join">
				  <button
					class="btn btn-sm btn-outline btn-successfull join-item"
					onclick="handleUpdateSubject(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(subject))}')))"
				  >
				 Save
				  </button>
				  <button
					class="btn btn-sm btn-outline btn-error join-item"
					onclick="handleResetSubject(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(subject))}')))"
				  >
				  Reset
				  </button>
				</div>
			</th>
			`;
      window.class_list.forEach((element) => {
        subject_row.querySelector("#class_name select").innerHTML +=
          `<option ${element.name === subject.class_name ? "selected" : ""} value="${element.name}">${element.name}</option>`;
      });
    }
  } else {
    alert("Something went wrong!");
  }
};
window.handleEditSubject = handleEditSubject;

// update subject handler
const handleUpdateSubject = (subject) => {
  const subject_row = document.getElementById(subject.id);
  let new_subject = {
    id: subject.id,
    name: subject_row.querySelector("#name input").value,
    display_name: `${subject_row.querySelector("#class_name select").value} - ${subject_row.querySelector("#name input").value}`,
    class_id: window.class_list.find(
      (cl) =>
        cl.name ==
        parseInt(subject_row.querySelector("#class_name select").value),
    ).id,
    class_name: parseInt(subject_row.querySelector("#class_name select").value),
    created_at: subject.created_at,
    updated_at: subject.updated_at,
  };
  // use `loadash` to compare structs
  if (_.isEqual(new_subject, subject)) {
    handleResetSubject(subject);
  } else {
    fetch("/api/subject", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(new_subject).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        // update the subject in the subject_list
        const index = window.subject_list.findIndex((sb) => sb.id === data.id);
        if (index !== -1) {
          window.subject_list[index] = { ...data };
        } else {
          alert("Something went Terribly Wrong!");
        }
        handleResetSubject(data);
      })
      .catch((_) => {
        handleResetSubject(subject);
        alert("Failed to update the Subject");
        throw new Error(`HTTP error! Status: ${res.status}`);
      });
  }
};
window.handleUpdateSubject = handleUpdateSubject;

// reset subject handler
const handleResetSubject = (subject) => {
  document.getElementById(subject.id).innerHTML = `
	<td>${subject.id}</td>
	<td class="max-w-[250px]">${subject.name}</td>
	<td>${subject.class_name}</td>
	<td data-timestamp="${subject.created_at}"></td>
	<td data-timestamp="${subject.updated_at}"></td>
	<th>
		<div class="join">
		  <button
			class="btn btn-sm btn-outline btn-secondary join-item"
			onclick="handleEditSubject('${subject.id}')"
		  >
		  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
		  </button>
		  <button
			class="btn btn-sm btn-outline btn-accent join-item"
			onclick="handleDeleteSubject('${subject.id}','${subject.name}')"
		  >
		  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
		  </button>
		</div>
	</th>
	`;
  updateUnixTimeStamps();
};
window.handleResetSubject = handleResetSubject;
