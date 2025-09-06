document.addEventListener('DOMContentLoaded', function() {
    // Theme toggle functionality
    const themeToggle = document.getElementById('themeToggle');
    const body = document.body;
    const icon = themeToggle.querySelector('i');
    
    themeToggle.addEventListener('click', () => {
        if (body.getAttribute('data-theme') === 'dark') {
            body.setAttribute('data-theme', 'light');
            body.classList.remove('dark-mode-bg');
            body.classList.add('light-mode-bg');
            icon.classList.remove('fa-moon');
            icon.classList.add('fa-sun');
        } else {
            body.setAttribute('data-theme', 'dark');
            body.classList.remove('light-mode-bg');
            body.classList.add('dark-mode-bg');
            icon.classList.remove('fa-sun');
            icon.classList.add('fa-moon');
        }
    });

    // Smooth scrolling for navigation links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });

    // Add scroll animation
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);

    // Observe elements for animation
    document.querySelectorAll('.skill-card, .project-card, .about-content > *').forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(20px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });

    const contactForm = document.getElementById('contactForm');
    const submitButton = document.getElementById('submitButton');
    const formStatus = document.getElementById('formStatus');
    
    if (contactForm) {
        contactForm.addEventListener('submit', async function(e) {
            e.preventDefault();
            
            // Show loading state
            submitButton.classList.add('btn-loading');
            submitButton.disabled = true;
            formStatus.className = 'form-status';
            formStatus.textContent = '';
            
            // Get form data and convert to JSON
            const formData = {
                name: this.querySelector('[name="name"]').value,
                email: this.querySelector('[name="email"]').value,
                message: this.querySelector('[name="message"]').value
            };
            
            // Basic validation
            if (!formData.name || !formData.email || !formData.message) {
                formStatus.textContent = 'Please fill in all fields.';
                formStatus.className = 'form-status error';
                submitButton.classList.remove('btn-loading');
                submitButton.disabled = false;
                return;
            }

            // Email validation
            const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
            if (!emailRegex.test(formData.email)) {
                formStatus.textContent = 'Please enter a valid email address.';
                formStatus.className = 'form-status error';
                submitButton.classList.remove('btn-loading');
                submitButton.disabled = false;
                return;
            }
            
            try {
                // Send to your Rust backend - UPDATED FOR JSON
                const response = await fetch('/contact', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(formData)
                });
                
                if (response.ok) {
                    // Success message
                    formStatus.textContent = 'Thank you! Your message has been saved. I\'ll get back to you soon.';
                    formStatus.className = 'form-status success';
                    contactForm.reset();
                    
                    // Optional: Clear success message after 5 seconds
                    setTimeout(() => {
                        formStatus.textContent = '';
                        formStatus.className = 'form-status';
                    }, 5000);
                    
                } else {
                    // Handle different HTTP error statuses
                    const errorText = await response.text();
                    throw new Error(`Server error: ${response.status} - ${errorText}`);
                }
            } catch (error) {
                // Error message
                formStatus.textContent = 'Failed to send message. Please try again later.';
                formStatus.className = 'form-status error';
                console.error('Form submission error:', error);
                
                // Optional: Clear error message after 5 seconds
                setTimeout(() => {
                    formStatus.textContent = '';
                    formStatus.className = 'form-status';
                }, 5000);
            } finally {
                // Remove loading state
                submitButton.classList.remove('btn-loading');
                submitButton.disabled = false;
            }
        });
    }


    // Add typing effect to terminal
    const terminalContent = document.querySelector('.terminal-content');
    if (terminalContent) {
        setTimeout(() => {
            terminalContent.innerHTML += '<p><span class="terminal-prompt">$ ready --to --code</span></p>';
            terminalContent.innerHTML += '<p>🚀 Starting development server...</p>';
        }, 2000);
    }
});