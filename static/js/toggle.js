(() => {
    document.querySelector('.toggle').addEventListener('click', () => {
        const checkbox = document.querySelector('.checkbox');
        checkbox.checked = !checkbox.checked;
        checkbox.dispatchEvent(new Event("change"));
    });
})();
