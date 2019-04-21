
(function() {
    const guestList = document.getElementById('guest-list');
    if (!guestList) return console.error('Failed to find #guest-list');
    const button = document.getElementById('add-guest-button');
    if (!button) return console.error('Failed to get #add-guest-button');
    button.addEventListener('click', () => {
        let ct = document.querySelectorAll('.guest-name-input').length;
        if (ct === 0) {
            guestList.appendChild(createHeading());
            guestList.appendChild(createLabel());
        }
        guestList.appendChild(createInputGroup(ct));
    });

    function createHeading() {
        let h = document.createElement('h4');
        h.setAttribute('class', 'guest-list-header');
        h.appendChild(document.createTextNode('Who Your Brining'));
        return h;
    }
    function createInputGroup(idx) {
        let group = document.createElement('div');
        group.setAttribute('class', 'input-group text');
        group.appendChild(createInput(idx));
        return group;
    }
    function createLabel() {
        let label = document.createElement('label');
        label.setAttribute('for', `guest-name`);
        label.appendChild(document.createTextNode('Name'));
        return label;
    }
    function createInput(idx) {
        let input = document.createElement('input');
        input.setAttribute('class', 'guest-name-input');
        input.setAttribute('type', 'text');
        input.setAttribute('id', `guest-name-${idx}`);
        input.setAttribute('name', 'guest-name[]');
        return input;
    }
})()