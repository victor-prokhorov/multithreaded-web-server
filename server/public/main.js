const fetchOrders = async () => {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 1000);
    try {
        const res = await fetch('/api/orders', { signal: controller.signal });
        if (!res.ok) {
            throw new Error(res.statusText);
        };
        const orders = await res.json();
        return orders;
    } catch (error) {
        console.error(error);
        if (error.name === 'AbortError') {
            throw new Error('fetch aborted', { cause: error });
        } else {
            throw new Error('failed to fetch orders', { cause: error });
        }
    } finally {
        clearTimeout(timeout);
    }
}

const renderOrders = (orders) => {
    const fragment = document.createDocumentFragment();
    const template = document.createElement('template');
    template.innerHTML = `<div><ul></ul></div>`;
    fragment.appendChild(template.content.cloneNode(true));
    const ul = fragment.querySelector('ul');
    orders.forEach(({ id, date, status }) => {
        const li = document.createElement('li');
        const d = new Date(date).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric', hour: 'numeric', minute: 'numeric', second: 'numeric' });
        li.textContent = `${id} ${d} ${status}`;
        ul.appendChild(li);
    });
    document.body.appendChild(fragment);
}

const main = async () => {
    document.body.appendChild(Object.assign(document.createElement('p'), { textContent: 'JavaScript loaded successfully' }));
    try {
        const orders = await fetchOrders()
        renderOrders(orders);
    } catch (error) {
        console.error(error);
        document.body.appendChild(Object.assign(document.createElement('p'), { textContent: `${error.message}: ${error.cause.message}` }));
    }
}

main();