use read_input::prelude::*;

fn get_winner(piles: Vec<u8>, mut sizes: Vec<u8>, cur_player: bool) -> bool
{
    for i in 0..sizes.len()
    {
        for j in 0..piles.len()
        {
            if piles[j] % sizes[i] == 0
            {
                let size = sizes[i];
                let mut temp_piles = piles.clone();
                temp_piles[j] = piles[j] / sizes[i];
                for _i in 0..size-1
                {
                    temp_piles.push(temp_piles[j]);
                }
                sizes.remove(i);
                if get_winner( temp_piles, sizes.clone(), !cur_player ) == cur_player
                {
                    return cur_player;
                }
                sizes.insert(i, size);
            }
        }
    }
    return !cur_player;
}

fn main() {
    let inp_pile: u8 = input().get();
    let size_count: u8;
    size_count = input().get();
    let mut sizes :Vec<u8> = [0 as u8; 0 as usize].to_vec();
    for _i in 0..size_count
    {
        sizes.push(input().get());
    }
    if get_winner(vec!(inp_pile), sizes, false)
    {
        println!("Second wins the game");
    }
    else
    {
        println!("First wins the game");
    }
}
