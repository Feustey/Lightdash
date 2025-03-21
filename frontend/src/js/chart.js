window.createChart = function(canvas, title, data, labels) {
    const config = {
        type: 'line',
        data: {
            labels: labels,
            datasets: [{
                label: title,
                data: data,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1
            }]
        },
        options: {
            responsive: true,
            plugins: {}
        }
    };

    return new Chart(canvas, config);
}; 