# Use an official Ubuntu base image 
FROM ubuntu:20.04 

# Install SSH server 
# Make sshd directory (required for SSH)
# Change default root password to root 
RUN apt-get update && \ 
    apt-get install -y openssh-server sudo && \
    mkdir /var/run/sshd && \
    echo 'root:root' | chpasswd 

# Configure SSH to accept password authentication (for now) 
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Expose the standard SSH port 
EXPOSE 22 

# Run the SSH server 
CMD ["/usr/sbin/sshd", "-D"]