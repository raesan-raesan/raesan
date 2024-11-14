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

// handle create_class_form submition
const handleCreateClassFormSubmit = () => {
  let create_class_form = document.getElementById("create_class_form");
  if (
    create_class_form.elements["name"].value.trim() === "" ||
    create_class_form.elements["name"].value.trim().length === 0
  ) {
    alert("Atleast enter something!");
  } else {
    fetch("/api/class", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "",
        name: parseInt(create_class_form.elements["name"].value),
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
        if (document.getElementById("create_class_modal")) {
          document.getElementById("create_class_modal").close();
          document.getElementById("class_table_body").innerHTML += `
				<tr id="${data.id}">
					<td>${data.id}</td>
					<td>${data.name}</td>
					<td data-timestamp="${data.created_at}"></td>
					<td data-timestamp="${data.updated_at}"></td>
					<th>
						<div class="join">
						  <button
							class="btn btn-sm btn-outline btn-secondary join-item"
							onclick="handleEditClass('${data.id}')"
						  >
						  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
						  </button>
						  <button
							class="btn btn-sm btn-outline btn-accent join-item"
							onclick="handleDeleteClass('${data.id}','${data.name}')"
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
window.handleCreateClassFormSubmit = handleCreateClassFormSubmit;

// handle create_class_from_json_input submition
document.getElementById("create_class_from_json_input").value = "";
const handleCreateClassFromJsonFormSubmit = () => {
  let create_class_from_json_input = document.getElementById(
    "create_class_from_json_input",
  );
  if (
    create_class_from_json_input.value.trim() === "" &&
    create_class_from_json_input.value.trim().length === 0
  ) {
    alert("Atleast enter something!");
  } else {
    fetch("/api/class/json", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(
        JSON.parse(create_class_from_json_input.value),
      ).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        if (document.getElementById("create_class_from_json_modal")) {
          document.getElementById("create_class_from_json_modal").close();
          data.forEach((element) => {
            document.getElementById("class_table_body").innerHTML += `
					<tr id="${element.id}">
						<td>${element.id}</td>
						<td>${element.name}</td>
						<td data-timestamp="${element.created_at}"></td>
						<td data-timestamp="${element.updated_at}"></td>
						<th>
							<div class="join">
							  <button
								class="btn btn-sm btn-outline btn-secondary join-item"
								onclick="editClass('${element.id}')"
							  >
							  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
							  </button>
							  <button
								class="btn btn-sm btn-outline btn-accent join-item"
								onclick="deleteClass('${element.id}','${element.name}')"
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
window.handleCreateClassFromJsonFormSubmit =
  handleCreateClassFromJsonFormSubmit;

// delete class handler
const handleDeleteClass = (class_id, class_name) => {
  let choice = confirm(`WARNING! Do you want to delete 'Class ${class_name}'`);
  if (choice == true) {
    fetch(`/api/class/${class_id}`, {
      method: "DELETE",
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.text();
      })
      .then((_) => {
        document.getElementById(class_id).remove();
      });
  }
};
window.handleDeleteClass = handleDeleteClass;

// edit class handler
const handleEditClass = (class_id) => {
  let _class = window.class_list.find((cl) => cl.id == class_id);
  if (_class) {
    let class_row = document.getElementById(_class.id);
    if (class_row) {
      class_row.innerHTML = `
			<td class="whitespace-nowrap">${_class.id}</td>
			<td id="name" class="whitespace-nowrap"><input type="number" placeholder="Name" value="${_class.name}" class="input input-bordered w-full max-w-xs min-w-[60px]"/></td>
			<th>
				<div class="join">
				  <button
					class="btn btn-sm btn-outline btn-successfull join-item"
					onclick="handleUpdateClass(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(_class))}')))"
				  >
				 Save
				  </button>
				  <button
					class="btn btn-sm btn-outline btn-error join-item"
					onclick="handleResetClass(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(_class))}')))"
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
window.handleEditClass = handleEditClass;

// update class handler
const handleUpdateClass = (_class) => {
  const class_row = document.getElementById(_class.id);
  let new_class = {
    id: _class.id,
    name: parseInt(class_row.querySelector("#name input").value),
    created_at: _class.created_at,
    updated_at: _class.updated_at,
  };
  // use `loadash` to compare structs
  if (_.isEqual(new_class, _class)) {
    handleResetClass(_class);
  } else {
    fetch("/api/class", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(new_class).trim(),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP error! Status: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        // update the class in the class_list
        const index = window.class_list.findIndex((cl) => cl.id === data.id);
        if (index !== -1) {
          window.class_list[index] = { ...data };
        } else {
          alert("Something went Terribly Wrong!");
        }
        handleResetClass(data);
      })
      .catch((_) => {
        handleResetClass(_class);
        alert("Failed to update the Class");
        throw new Error(`HTTP error! Status: ${res.status}`);
      });
  }
};
window.handleUpdateClass = handleUpdateClass;

// reset class handler
const handleResetClass = (_class) => {
  document.getElementById(_class.id).innerHTML = `
	<td>${_class.id}</td>
	<td>${_class.name}</td>
	<td data-timestamp="${_class.created_at}"></td>
	<td data-timestamp="${_class.updated_at}"></td>
	<th>
		<div class="join">
		  <button
			class="btn btn-sm btn-outline btn-secondary join-item"
			onclick="handleEditClass('${_class.id}')"
		  >
		  <span class="iconify mdi--edit-outline w-[22px] h-[22px]"></span>
		  </button>
		  <button
			class="btn btn-sm btn-outline btn-accent join-item"
			onclick="handleDeleteClass('${_class.id}','${_class.name}')"
		  >
		  <span class="iconify mdi--bin-outline w-[22px] h-[22px]"></span>
		  </button>
		</div>
	</th>
	`;
  updateUnixTimeStamps();
};
window.handleResetClass = handleResetClass;
