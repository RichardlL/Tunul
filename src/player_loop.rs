/*
Tunnul
Copyright (c) 2015, Richard Lettich
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. The name of the author may not be used to endorse or promote products
   derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

--------------------------------------------------------------------------

THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT,
NEITHER APPROVED NOR ASSOCIATED WITH MOJANG.
*/

// Array of packet handler, instead of doing a huge match
// We just take the packet ID, packet_handler[packet.id]
// (packet id 0 is position 0),
/*static packet_handler = [
        recv_keep_alive(),
        chat_message(),
        use_entity(),
        is_flying(),
        position_update(),
        look_update(),
        position_and_look_update(),
        player_digging(),
        block_placement(),
        held_item_update(),
        action(),
        vehichle_steer(),
        close_window(),
        click_inventory_slot(),
        confirm_transaction(),
        creative_inventory(),
        enchant_item(),
        sign_set(),
        player_abilities(),
        tab_complete(),
        client_settings(),
        spawn_request(),
        plugin_message(),
        spectate(),
        resource_pack_status(),
        ];
*/
use player;
use packet;
use std::net::TcpStream;
pub fn player_loop(mut stream:  Box<TcpStream>) {
    let mut player = player::Player::from_stream(stream);

    player.confirm_login();
    player.join_game();
    player.send_spawn();
    player.send_location();
    loop {}
}
   /* loop {
        let packet = packet::Packet::new(&mut stream);
        match packet.id {
            0..packet_handler.len() => (),
            _ => player.kick_player("Invalid Packet ID,  Hacking?"),
        }
        match player.packet_handler[packet.id](&packet) {
            Ok => (),
            Err(e) => { player.kick_player(e); return; },
        }
    }
}

impl Player {
    recv_keep_alive(&packet) -> Result {
        Ok
    }
    fn chat_message(&packet) {
        let message = try!(inputpacket.get_string());
    }
}*/

