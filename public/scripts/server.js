window.addEventListener("load", loadHandler);


function loadHandler(event) {
    console.log("Load handling...");
    
    b = document.getElementById('mc_bedrock');
    j = document.getElementById('mc_java');

    
    b.addEventListener('click', async function(event) {
        // const name = document.getElementById('name').value;
        // const email = document.getElementById('email').value;
        a = get_action(b);
        console.log(a);

        const data = {
            server: 'bedrock',
            action: a
        };
        post_data(data);
    });
    j.addEventListener('click', async function(event) {
        // const name = document.getElementById('name').value;
        // const email = document.getElementById('email').value;
        
        var a = get_action(j);
        console.log(a);

        const data = {
            server: 'java',
            action: a
        };
        post_data(data);
    });

}

function get_action(button) {
    switch (parseInt(button.value)) {
        case 0:
            return 'start';
        case 1:
            return 'close';
        case 2:
            return 'restart';
        default:
            return 'other';
    }
}

async function post_data(data) {
    try {
        const response = await fetch('http://72.181.111.213:3000/server', { // Replace with your server's URL
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
    } catch (error) {
        // document.getElementById('response').innerText = 'Error: ' + error.message;
        console.error('Error :(');
    }
}
