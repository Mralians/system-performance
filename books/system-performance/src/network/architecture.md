# Architecture

## Software

1. Networking software includes the **network stack**, **TCP**, and **device drivers**.

### Network Stack

1. On modern kernels the stack is multithreaded, and inbound packets can be processed by multiple CPUs.

#### Linux

1. On Linux systems, the network stack is a core kernel component.

2. device drivers are additional modules.

3. Packets are passed through these kernel components as the struct `sk_buff` (socket buffer) data type.

4. there may also be queueing in the IP layer (not pictured) for packet reassembly.

5. **Application**: This is the highest level where user applications operate. Applications use network services to send and receive data over the network.

   **Libraries**: These provide a set of functions for the application to use for networking, abstracting the complexity of direct system calls.

   **System Calls**: These are the interfaces provided by the operating system kernel that applications call to perform network operations like send and receive data.

   **Kernel**: The central component of the OS that manages operations between hardware and software.

   - **VFS (Virtual File System)**: It provides an abstraction layer for file system operations and might also be involved in network operations when data is written to or read from network sockets as if they were files.
   - **Socket**: A network socket is an endpoint for sending or receiving data across a computer network. Sockets have buffers for sending and receiving data.
   - **TCP/UDP/ICMP**: These are different protocols for handling network communication.
     - TCP (Transmission Control Protocol) provides reliable, ordered, and error-checked delivery of a stream of data.
     - UDP (User Datagram Protocol) is a simpler, connectionless Internet protocol that allows sending datagrams without establishing a connection.
     - ICMP (Internet Control Message Protocol) is used for diagnostic purposes and error reporting, not typically for sending and receiving application data.
   - **IP (Internet Protocol)**: This protocol is designed for sending packets across the network using IP addresses to identify the source and destination.
   - **Queuing Discipline (qdisc)**: This is a set of rules for how packets should be processed for transmission, including ordering, prioritizing, and scheduling packets.
   - **Driver Queue**: Each network interface card (NIC) driver has its own queue to buffer packets before they are transmitted or after they are received.
   - **NIC / Virtual Device**: The physical or virtual device that connects a computer to a network. The NIC has its own set of drivers that interact with the rest of the computer's hardware and software.
   - **Device Drivers**: These are specific software that controls the hardware device, in this case, the network interface card.

6. when you're writing a network driver, part of your job is to handle the frames that are received from the network. Here's how it generally works:

   1. **Receiving Frames**: The network interface card (NIC) receives frames from the physical medium (like an Ethernet cable) and places them into its hardware buffer.
   2. **Interrupt**: Once the frame is received, the NIC typically generates an interrupt to signal the CPU that data has arrived.
   3. **Driver's Role**: The network driver, which you would be writing, responds to this interrupt. It reads the frame from the NIC's hardware buffer into system memory.
   4. **Passing Up**: After the frame is in system memory, the driver then hands it off to the operating system's networking stack, which processes it at various layers (like IP, TCP/UDP, etc.) until it reaches the application layer if it's incoming data.
   5. **Sending Frames**: For outgoing data, the process is reversed. The application sends data down the stack, which eventually hands it off to the network driver. The driver then places this data into the NIC's hardware buffer for transmission on the network.
   6. **Buffer Management**: Network drivers often implement or interact with a ring buffer or a similar data structure in system memory to efficiently manage the packets that are waiting to be processed.

   Writing a network driver involves managing these buffers, handling interrupts, and interfacing with the operating system's networking subsystems, as well as dealing with the specific hardware operations of the network device. It requires a good understanding of both the hardware and the software stack that the driver interacts with.

7. `struct sk_buff`, commonly known as `sk_buff`, is a data structure in the Linux kernel networking stack that represents network packets. It holds the packet's content and its associated metadata, such as timestamps, network headers, and the origin and destination of the packet. `sk_buff` is used to manage and manipulate packets as they flow through the network stack, with functions provided by the kernel for common operations like resizing or modifying the data buffer. It's a fundamental structure that encapsulates both the packet's data and the control information used by the networking subsystem.

### TCP Connection Queues

**1. Backlog Queues:**
   - **Purpose:** Manage bursts of inbound TCP connections.
   - **Types:**
     - **SYN Backlog:** For connections in the TCP handshake phase.
     - **Listen Backlog:** For established connections awaiting application acceptance.

**2. Queue Management Evolution:**
   - **Early Systems:** Used a single queue, **vulnerable to SYN flood attacks**.
   - **SYN Flood Attack:** Denial of Service (DoS) attack involving numerous bogus SYN requests to a TCP port, blocking legitimate connections.

**3. Improved Management with Two Queues:**
   - **Dual Queue System:** Separates potentially connections from legitimate ones.
   - **Benefits:**
     - **Staging for Unverified Connections:** The SYN Backlog acts as a filter.
     - **Established Connections:** Only verified connections reach the Listen Backlog.
     - **Optimized for Attack Mitigation:** The SYN Backlog is lengthened and optimized to handle SYN floods with minimal metadata storage.
     - **SYN Cookies:** SYN cookies are a method of handling SYN requests without having to allocate significant resources for each connection. Instead of storing each incoming SYN request in a queue, the server sends back a SYN-ACK response with a specially crafted sequence number (the "cookie"). This sequence number is generated based on the IP address, port number, and other characteristics of the incoming SYN request.
     
       **Bypassing the First Queue:**  the server does not need to store the state of each incoming SYN request in the SYN backlog. Instead, it relies on the client to respond correctly to the SYN-ACK with this special sequence number. If the client is legitimate and completes the handshake using the cookie (sequence number), the server can then establish the connection. This way, the server doesn't waste resources on connections that are never completed, as would be the case in a SYN flood attack.

### TCP Buffering

1. **Send and Receive Buffers:**
   - **Purpose:** These buffers are used to temporarily store data before it is sent (send buffer) or after it is received (receive buffer) over a network connection.
   - **Location:** They are associated with each socket, which is an endpoint for sending or receiving data in a network connection.
2. **Tunable Buffer Sizes:**
   - **Customization:** The size of both send and receive buffers can be adjusted (tuned) to suit specific needs.
   - **Throughput vs. Memory Trade-off:** Increasing the size of these buffers can improve data throughput (the rate at which data is successfully transferred over the network). However, larger buffer sizes also mean more of the computer's main memory (RAM) is used per connection.
3. **Asymmetric Buffer Sizing:**
   - **Adaptation to Server Role:** One buffer can be made larger than the other, depending on the server’s expected usage. For instance, if a server is primarily sending data, the send buffer may be increased in size compared to the receive buffer, and vice versa.
4. **Dynamic Buffer Sizing by the Linux Kernel:**
   - **Automatic Adjustment:** The Linux kernel can dynamically adjust the size of these buffers based on the activity of the connection.
   - **Tuning Parameters:** The kernel allows for the tuning of buffer sizes, including setting minimum, default, and maximum sizes.
5. the send and receive buffers for TCP sockets, which are mentioned in the context of improving data throughput, are different from the buffers shown in the output of the `free -h` command on Linux systems.
   1. **TCP Send and Receive Buffers:**
      - These buffers are specifically allocated for each TCP socket connection.
      - Their sizes determine how much data can be temporarily stored while being sent or received over that particular socket.
      - The sizes of these buffers can be tuned for performance optimization and are managed by the TCP stack within the kernel.
   2. **Buffers in `free -h` Command:**
      - The `free -h` command in Linux displays the total amount of free and used physical and swap memory in the system, as well as the buffers and cache used by the kernel.
      - The "buffers" shown in `free -h` refers to memory used by the kernel to buffer block devices (like hard drives). This helps in speeding up access to disk data and is unrelated to network sockets.
      - These are general-purpose buffers used by the kernel for various system activities and are not directly related to the send and receive buffers of TCP sockets.

### Segmentation Offload: GSO and TSO

1. **Segmentation Offload:**

   - **Purpose:** This is a technique used to reduce the overhead of the network stack (the software that handles network communication) in an operating system.

     To avoid the network stack overheads of sending many small packets Linux uses generic segmentation offload (GSO) .

2. **Maximum Segment Size (MSS):**

   - **Definition:** MSS is the largest size of a packet or segment that can be sent in a TCP connection. For many networks, the MSS is typically around 1500 bytes, which is the standard Ethernet frame size.

3. **Generic Segmentation Offload (GSO):**

   - **Function:** GSO allows the operating system to send large packets (up to 64 Kbytes, referred to as “super packets”) to the network device.
   - **Process:** These super packets are then divided into smaller segments, each fitting within the MSS, just before they are delivered to the network device for transmission.

4. **TCP Segmentation Offload (TSO):**

   - **Integration with Network Interface Cards (NICs):** When the NIC and its driver support TSO, the task of splitting the large packets into MSS-sized segments is offloaded to the NIC hardware.
   - **Advantage:** This offloading significantly reduces the processing load on the server's CPU and improves the throughput of the network stack.

5. **Generic Receive Offload (GRO):**

   - **Complementary Technique to GSO:** GRO is similar to GSO but works for incoming packets. It allows the system to process fewer, larger packets, which reduces CPU overhead.
   - **Implementation:** Both GRO and GSO are implemented in the kernel software.

6. **Implementation of TSO:**

   - **Hardware-Based:** TSO is implemented in the NIC hardware, differentiating it from GSO and GRO, which are software-based.



### Queueing Discipline

1. 
