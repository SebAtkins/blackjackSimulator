#![allow(non_snake_case)]

use rand::Rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::env;

struct Game {
    deck: Vec<usize>,
    pos: usize,
    split: usize,
    dealer: Vec<usize>,
    players: Vec<Vec<usize>>,
    playerOwner: Vec<usize>,
    playerTotals: Vec<usize>
}

struct Moves {
    stand: bool,
    hit: bool,
    split: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let deckCount: usize = args[1].to_string().parse().unwrap();
    let playerCount: usize = args[2].to_string().parse().unwrap();

    let mut game = initGame(deckCount, playerCount);

    // Run game for all players
    playHands(&mut game);

    // Draw cards for dealer
    playDealer(&mut game);

    // Determine which players won
    findWinners(&game);
}

fn findWinners(game: &Game) {
    // Find dealer score (to beat)
    let mut temp = 0;
    let mut dealerTotal = 0;
    let mut aceCount = 0;

    for j in 0..game.dealer.len() {
        temp = game.dealer[j] & 15;
        if temp <= 10 {
            dealerTotal += temp;
        } else {
            if temp < 14 {
                dealerTotal += 10;
            } else {
                dealerTotal += 11;
                aceCount += 1;
            }
        }
    }

    // Account for ace being 11 or 1
    while dealerTotal > 21 && aceCount > 0 {
        dealerTotal -= 10;
        aceCount -= 1;
    }

    // Check if dealer is bust
    if dealerTotal <= 21 {
        // Display winning players
        let mut wonBool = "poop";
        for i in 0..game.players.len() {
            wonBool =  if game.playerTotals[i] > dealerTotal && game.playerTotals[i] <= 21 {"Won"} else {"Lost"};

            println!("Player {}: {} with a score of {}", game.playerOwner[i] + 1, wonBool, game.playerTotals[i]);
        }
    } else {
        // All players won if they are not bust
        let mut wonBool = "poop";
        for i in 0..game.players.len() {
            wonBool =  if game.playerTotals[i] <= 21 {"Won"} else {"Lost"};

            println!("Player {}: {} with a score of {}", game.playerOwner[i] + 1, wonBool, game.playerTotals[i]);
        }
    }
}

fn playDealer(game: &mut Game) {
    // Show dealer's full hand
    showHands(game, false);

    // Find total of dealer's hand
    let mut temp = 0;
    let mut total = 0;
    let mut aceCount = 0;

    for j in 0..game.dealer.len() {
        temp = game.dealer[j] & 15;
        if temp <= 10 {
            total += temp;
        } else {
            if temp < 14 {
                total += 10;
            } else {
                total += 11;
                aceCount += 1;
            }
        }
    }

    // Account for ace being 11 or 1
    while total > 21 && aceCount > 0 {
        total -= 10;
        aceCount -= 1;
    }

    println!("Dealer's Total: {}", total);

    // Draw cards until the total is <= 17
    while total < 17 {
        // Draw card for dealer
        let val = game.deck[game.pos];
        game.dealer.push(val);
        game.pos += 1;

        // Recalculate total
        temp = 0;
        total = 0;
        aceCount = 0;

        for j in 0..game.dealer.len() {
            temp = game.dealer[j] & 15;
            if temp <= 10 {
                total += temp;
            } else {
                if temp < 14 {
                    total += 10;
                } else {
                    total += 11;
                    aceCount += 1;
                }
            }
        }
    
        // Account for ace being 11 or 1
        while total > 21 && aceCount > 0 {
            total -= 10;
            aceCount -= 1;
        }
    }

    // Show dealer's finished hand
    showHands(game, false);
    println!("Dealer's Total: {}", total);
    
}

fn playHands(game: &mut Game) {
    let mut i = 0;
    let mut length = game.players.len();
    while i < length {
        // Loop until player is bust or stood
        let mut playing = true;
        let mut firstMove = true;
        while playing {
            showHands(game, true);
            
            // Find player total
            let mut total = 0;
            let mut temp = 0;
            let mut aceCount = 0;
            for j in 0..game.players[i].len() {
                temp = game.players[i][j] & 15;
                if temp <= 10 {
                    total += temp;
                } else {
                    if temp < 14 {
                        total += 10;
                    } else {
                        total += 11;
                        aceCount += 1;
                    }
                }
            }

            // Account for ace being 11 or 1
            while total > 21 && aceCount > 0 {
                total -= 10;
                aceCount -= 1;
            }

            // Save player total
            game.playerTotals[i] = total;

            if total <= 21 {
                println!("Player {} total: {}", game.playerOwner[i] + 1, total);
            } else {
                println!("Bust!");
                playing = false;
                i += 1;
                length = game.players.len();
                continue;
            }

            // Find valid moves
            let mut moves = Moves{stand: true, hit: false, split: false};

            if game.players[i].len() == 2 && firstMove && game.players[i][0] & 15 == game.players[i][1] & 15 {
                moves.split = true;
            }
            if total < 21 {
                moves.hit = true;
            }

            // Show possible moves
            println!("Player {} valid moves: ", game.playerOwner[i] + 1);
            if moves.stand {
                println!("1: Stand ");
            }
            if moves.hit {
                println!("2: Hit ");
            }
            if moves.split {
                println!("3: Split");
            }

            // Get player's move
            let selectedMove = getMove(&moves);

            // Perform player's move
            if selectedMove == 1 {
                playing = false;
            } else if selectedMove == 2 {
                drawCard(game, i);
            } else {
                splitHand(game, i)
            }

            if firstMove {
                firstMove = false;
            }
        }
        
        i += 1;
        length = game.players.len();
    }
}

fn splitHand(game: &mut Game, playerPos: usize) {
    // Create new array for second deck and assign the correct player owner
    game.players.insert(playerPos + 1, Vec::new());
    game.playerOwner.insert(playerPos + 1, game.playerOwner[playerPos]);
    game.playerTotals.insert(playerPos + 1, 0);

    // Move cards from old hand into new hand
    let card = game.players[playerPos].pop().unwrap();
    game.players[playerPos + 1].push(card);
}

fn drawCard(game: &mut Game, playerPos: usize) {
    let val = game.deck[game.pos];
    game.players[playerPos].push(val);
    game.pos += 1;
}

fn getMove(moves: &Moves) -> usize {
    // Get players move
    let mut line = String::new();
    println!("Please enter move ID: ");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    
    // Check if move is stand
    if line.trim() == "1" {
        return 1;
    } else {
        // Check if move is hit 
        if line.trim() == "2" && moves.hit {
            return 2;
        } else {
            // Check if move is split
            if line.trim() == "3" && moves.split {
                return 3;
            } else {
                println!("Please enter valid move");
                return getMove(moves);
            }
        }
    }
}

fn showHands(game: &Game, hidden: bool) {
    // Print dealer's cards
    let mut dealerHand = "".to_owned();
    dealerHand.push_str(&decodeCard(game.dealer[0]));
    // Hide dealer's second card before their turn
    if hidden {
        dealerHand.push_str(", *");
    } else {
        for i in 1..game.dealer.len() {
            dealerHand.push_str(", ");
            dealerHand.push_str(&decodeCard(game.dealer[i]));
        }
    }
    println!("Dealer's Hand: {}", dealerHand);
    
    // Print players' cards
    for i in 0..game.players.len() {
        let mut playerHand = "".to_owned();
        playerHand.push_str(&decodeCard(game.players[i][0]));
        for j in 1..game.players[i].len() {
            playerHand.push_str(", ");
            playerHand.push_str(&decodeCard(game.players[i][j]));
        }

        println!("Player {}'s Hand: {}", game.playerOwner[i] + 1, playerHand);
    }
}

fn decodeCard(card: usize) -> String {
    let suit = match (card & 48) >> 4 {
        0 => "Hearts",
        1 => "Clubs",
        2 => "Spades",
        3 => "Diamonds",
        _ => "Bug",
    };

    let mut card = match (card & 15).to_string().as_str() {
        "11" => "J".to_owned(),
        "12" => "Q".to_owned(),
        "13" => "K".to_owned(),
        "14" => "A".to_owned(),
        x => x.to_owned(),
    };

    card.push_str(" of ");
    card.push_str(suit);

    return card;
}

// Initialise the decks and draw the initial cards
fn initGame(decks: usize, players: usize) -> Game {
    // Create list of cards in specified decks
    // 2 - 10: Normal
    // 11: J
    // 12: Q
    // 13: K
    // 14: A
    let mut cards = Vec::new();

    for _ in 0..decks {
        for suit in 0..4 {
            for card in 2..15 {
                cards.push(card ^ (suit << 4));
            }
        }
    }

    // Shuffle deck
    let mut irs = Irs::default();
    let mut rng = rand::thread_rng();
    match irs.shuffle(&mut cards, &mut rng) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    // Find cut point
    let notUsed = rand::thread_rng().gen_range(60..76);
    let cutPoint = decks * 52 - notUsed;

    // Create hands for players
    let mut playerHands: Vec<Vec<usize>> = Vec::new();
    for _ in 0..players {
        playerHands.push(Vec::new());
    }
    let playerHandOwner: Vec<usize> = Vec::from_iter(0..players);
    let playerHandTotal: Vec<usize> = Vec::from_iter(0..players);

    let mut game = Game {
        deck: cards,
        pos: 0,
        split: cutPoint,
        dealer: Vec::new(),
        players: playerHands,
        playerOwner: playerHandOwner,
        playerTotals: playerHandTotal
    };

    // Draw cards
    for _ in 0..2 {
        // Draw players' cards
        for i in 0..players {
            game.players[i].push(game.deck[game.pos]);
            game.pos += 1;
        }

        // Draw dealer's card
        game.dealer.push(game.deck[game.pos]);
        game.pos += 1;
    }

    return game;
}
