#[repr(u16)]
pub enum OpCodes {
    /** This is an ArtPoll packet, no other data is contained in this UDP packet. */
    Poll = 0x2000,
    /** This is an ArtPollReply Packet. It contains device status information. */
    PollReply = 0x2100,
    /** Diagnostics and data logging packet. */
    DiagData = 0x2300,
    /** Used to send text based parameter commands. */
    Command = 0x2400,
    /** This is an ArtDmx data packet. It contains zero start code DMX512 information for a single Universe. */
    Dmx = 0x5000,
    /** This is an ArtNzs data packet. It contains non-zero start code (except RDM) DMX512 information for a single Universe. */
    Nzs = 0x5100,
    /** This is an ArtSync data packet. It is used to force synchronous transfer of ArtDmx protocol to a node’s output. */
    Sync = 0x5200,
    /** This is an ArtAddress packet. It contains remote programming information for a Node. */
    Address = 0x6000,
    /** This is an ArtInput packet. It contains enable – disable data for DMX inputs. */
    Input = 0x7000,
    /** This is an ArtTodRequest packet. It is used to request a Table of Devices (ToD) for RDM discovery. */
    TodRequest = 0x8000,
    /** This is an ArtTodData packet. It is used to send a Table of Devices (ToD) for RDM discovery. */
    TodData = 0x8100,
    /** This is an ArtTodControl packet. It is used to send RDM discovery control messages. */
    TodControl = 0x8200,
    /** This is an ArtRdm packet. It is used to send all non discovery RDM messages. */
    Rdm = 0x8300,
    /** This is an ArtRdmSub packet. It is used to send compressed, RDM Sub-Device data. */
    RdmSub = 0x8400,
    /** This is an ArtVideoSetup packet. It contains video screen setup information for nodes that implement the extended video features. */
    VideoSetup = 0xa010,
    /** This is an ArtVideoPalette packet. It contains colour palette setup information for nodes that implement the extended video features. */
    VideoPalette = 0xa20,
    /** This is an ArtVideoData packet. It contains display data for nodes that implement the extended video features. */
    VideoData = 0xa040,
    /** This packet is deprecated. */
    MacMaster = 0xf000,
    /** This packet is deprecated. */
    MacSlave = 0xf100,
    /** This is an ArtFirmwareMaster packet. It is used to upload new firmware or firmware extensions to the Node. */
    FirmwareMaster = 0xf200,
    /** This is an ArtFirmwareReply packet. It is returned by the node to acknowledge receipt of an ArtFirmwareMaster packet or ArtFileTnMaster packet. */
    FirmwareReply = 0xf300,
    /** Uploads user file to node. */
    FileTnMaster = 0xf400,
    /** Downloads user file from node. */
    FileFnMaster = 0xf500,
    /** Server to Node acknowledge for download protocol. */
    FileFnReply = 0xf600,
    /** This is an ArtIpProg packet. It is used to re- programme the IP address and Mask of the Node. */
    IpProg = 0xf800,
    /** This is an ArtIpProgReply packet. It is returned by the node to acknowledge receipt of an ArtIpProg packet. */
    IpProgReply = 0xf900,
    /** This is an ArtMedia packet. It is Unicast by a Media Server and acted upon by a Controller. */
    Media = 0x9000,
    /** This is an ArtMediaPatch packet. It is Unicast by a Controller and acted upon by a Media Server. */
    MediaPatch = 0x9100,
    /** This is an ArtMediaControl packet. It is Unicast by a Controller and acted upon by a Media Server. */
    MediaControl = 0x9200,
    /** This is an ArtMediaControlReply packet. It is Unicast by a Media Server and acted upon by a Controller. */
    MediaContrlReply = 0x9300,
    /** This is an ArtTimeCode packet. It is used to transport time code over the network. */
    TimeCode = 0x9700,
    /** Used to synchronise real time date and clock */
    TimeSync = 0x9800,
    /** Used to send trigger macros */
    Trigger = 0x9900,
    /** Requests a node's file list */
    Directory = 0x9a00,
    /** Replies to OpDirectory with file list */
    DirectoryRely = 0x9b00
}

impl OpCodes {
    pub fn from(bytes: u16) -> Option<OpCodes> {
        match bytes {
            0x5000 => Some(OpCodes::Dmx),
            _ => None
        }
    }
}
