window.addEventListener("load", loadHandler);
var num_items = 0;

function loadHandler(event) {
    const addPerson = document.getElementById("add_person");
    addPerson.addEventListener("click", function () {
        console.log("PopUp!");
        const popup = document.getElementById("popup");
        popup.style.display = "flex";
    });
    const new_item = document.getElementById("add_item");
    new_item.addEventListener("click", function () {
        console.log("New Collapsable");
        const items = document.getElementById("item_container");
        num_items++;
        items.insertAdjacentHTML(
            "beforeend",
            "<div class='collapsible-header' onclick='toggleCollapse(this)'>" +
                "<span>Item " +
                num_items +
                "</span>" +
                "<span class='arrow' id='arrow'>▼</span>" +
                "</div>" +
                "<div class='collapsible-content' id='content_" +
                num_items +
                "'> " +
                "<div class='content-inner'>" +
                "<div class='event'> <label>Event Name</label> <input type='text' id='event'> </div>" +
                "<div class='days'>" +
                "<div class='check_day'> <label>Sun</label> <input type='checkbox' id='day_0' value='0'> </div>" +
                "<div class='check_day'> <label>Mon</label> <input type='checkbox' id='day_1' value='1'> </div>" +
                "<div class='check_day'> <label>Tue</label> <input type='checkbox' id='day_2' value='2'> </div>" +
                "<div class='check_day'> <label>Wed</label> <input type='checkbox' id='day_3' value='3'> </div>" +
                "<div class='check_day'> <label>Thu</label> <input type='checkbox' id='day_4' value='4'> </div>" +
                "<div class='check_day'> <label>Fri</label> <input type='checkbox' id='day_5' value='5'> </div>" +
                "<div class='check_day'> <label>Sat</label> <input type='checkbox' id='day_6' value='6'> </div>" +
                "</div>" +
                "<div class='times'>" +
                "<div class='start_end'>" +
                "<label>Start Time <label>" +
                "<input type='time' id='start'>" +
                "<label>End Time <label>" +
                "<input type='time' id='end'>" +
                "</div>",
        );
    });
}

function toggleCollapse(element) {
    console.log("Toggled!");
    const arrow = element.querySelector("#arrow");
    const content = element.nextElementSibling;
    console.log(content);
    arrow.classList.toggle("rotated");
    content.classList.toggle("open");
}

function submitForm() {
    const sched = {
        name: document.getElementById("name").value,
        act: [],
    };
    for (let i = 1; i <= num_items; i++) {
        const content = document.getElementById(`content_${i}`);
        const temp = {
            title: namecontent.querySelector("title").value,
            start_time: namecontent.querySelector("start").value,
            end_time: namecontent.querySelector("end").value,
            days: namecontent.querySelector("days").value,
        };
        sched.act[i - 1] = temp;
    }
    console.log(sched);
    post_data(sched);
}

async function post_data(data) {
    try {
        const response = await fetch("https://72.181.111.213:3000/schedule", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const server_data = await response.json();
        if (!server_data.error_msg) {
            loadServers();
        } else {
            handlePopup(server_data);
        }
    } catch (error) {
        // document.getElementById('response').innerText = 'Error: ' + error.message;
        console.error("Error :( " + error.message);
    }
}
