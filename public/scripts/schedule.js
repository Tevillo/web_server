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
        items.innerHTML +=
            "<div class='collapsible-header' onclick='toggleCollapse(this)'>" +
            "<span>Item " +
            num_items +
            "</span>" +
            "<span class='arrow' id='arrow_" +
            num_items +
            "'>▼</span>" +
            "</div>" +
            "<div class='collapsible-content' id='content_" +
            num_items +
            "'> " +
            "<div class='content-inner'> <h3>Hidden Content</h3> <p>This is the collapsible content that can be shown or hidden.</p> <p>You can put any HTML content here - text, images, lists, or other elements.</p> <ul> <li>Item 1</li> <li>Item 2</li> <li>Item 3</li> </ul> </div> ";
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
    console.log("Form Submitted!");
}
