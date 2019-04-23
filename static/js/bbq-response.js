(function() {
    let container = document.getElementById('bbq-response-container');
    if (!container) return console.error('Unable to find #bbq-response-container');
    let fullQuery = location.search.substr(1);
    if (location.pathname.indexOf('success') > -1) {
        success();
    } else {
        error();
    }
    function success() {
        let div = document.createElement('div');
        div.setAttribute('class', 'success-message-info');
        let pairs = fullQuery.split('&');
        let rsvp = {};
        for (let pair of pairs) {
            let parts = pair.split('=');
            if (parts.length != 2) {
                return console.error('Failed to parse url query', parts);
            }
            rsvp[decodeURIComponent(parts[0])] = decodeURIComponent(parts[1]);
        }
        let message = document.createElement('span');
        message.setAttribute('class', 'rsvp-message');
        let attending;
        if (rsvp.regrets && rsvp.regrets > 0) {
            attending = 'not coming... we\'ll miss you';
        } else {
            let party = '';
            let partySize = parseInt(rsvp.party_size);
            if (partySize > 0) {
                if (partySize === 1) {
                    party = ' and bringing one person';
                } else {
                    party = ` and bringing ${rsvp.party_size} people`
                }
            }
            attending = `coming${party}`;
        }
        message.appendChild(document.createTextNode(`We got ${rsvp.first_name} ${rsvp.last_name} down as ${attending}!`));
        container.appendChild(message);
    }
    function error() {
        let span = document.createElement('span');
        span.setAttribute('class', 'error message');
        let parts = fullQuery.split('=');
        if (parts.length > 2) {
            console.error('More than 2 parts for an error?', parts);
        }
        if (parts.length < 2) {
            return console.error('Too few parts in query string...', parts);
        }
        let msg = document.createTextNode(decodeURIComponent(parts[1]));
        span.appendChild(msg);
        container.appendChild(span);
    }
})();