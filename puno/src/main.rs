mod card;

use card::{Card, generate_cards};
use futures::executor::block_on;
use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;
use std::task::Poll;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let cards: Vec<Card> = generate_cards();

	let local_key = identity::Keypair::generate_ed25519();
	let local_peer_id = PeerId::from(local_key.public());

	println!("Peer Id: {}", local_peer_id.clone());

	let transport = block_on(libp2p::development_transport(local_key))?;

	let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

	let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    block_on(future::poll_fn(move |cx| loop {
        match swarm.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => match event {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
                SwarmEvent::Behaviour(event) => println!("{:?}", event),
                _ => {}
            },
            Poll::Ready(None) => return Poll::Ready(()),
            Poll::Pending => return Poll::Pending
        }
    }));

	Ok(())
}
