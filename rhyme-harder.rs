use std;

#[test]
fn testLettersDontRepeat_passIfTrue()
{
	assert lettersDontRepeat(str::chars("ab"));
}

#[test]
fn testLettersDontRepeat_failIfFalse()
{
	assert !lettersDontRepeat(str::chars("aba"));
}

#[test]
fn testAFirstIfPresent_passIfIs()
{
	assert aIsFirstIfPresent(str::chars("ab"));
}

#[test]
fn testAFirstIfPresent_passIfNotPresent()
{
	assert aIsFirstIfPresent(str::chars("bc"));
}

#[test]
fn testAFirstIfPresent_failIfIsNot()
{
	assert !aIsFirstIfPresent(str::chars("ba"));
}

#[test]
fn testZLastIfPresent_passIfIs()
{
	assert zIsLastIfPresent(str::chars("az"));
}

#[test]
fn testZLastIfPresent_passIfNotPresent()
{
	assert zIsLastIfPresent(str::chars("ab"));
}

#[test]
fn testZLastIfPresent_failIfIsNot()
{
	assert !zIsLastIfPresent(str::chars("za"));
}

#[test]
fn testContainsAllLetters_succeedIfDoes()
{
	assert containsAllLetters(str::chars("abcdefghijklmnopqrstuvwxyz"));
}

#[test]
fn testContainsAllLetters_failIfNot()
{
	assert !containsAllLetters(str::chars("za"));
}

#[test]
fn testLettersRhyme_succeedIfDo()
{
	assert lettersRhyme('a', 'h');
}

#[test]
fn testLettersRhyme_failIfDoNot()
{
	assert !lettersRhyme('a', 'b');
}
#[test]
fn testPathIsValid_succeedIfIs()
{
	assert pathIsValid(str::chars("a"));
	assert pathIsValid(str::chars("ab"));
	assert pathIsValid(str::chars("az"));
}

fn pathIsValid(path : [char]) -> bool
{
	let mut isValid = lettersDontRepeat(path)
	&& aIsFirstIfPresent(path)
	&& zIsLastIfPresent(path);
	let mut index = 0u;
	while index < vec::len(path) - 1u
	{
		isValid &= !lettersRhyme(path[index], path[index + 1u]);
		index += 1u;
	}
	isValid
}

fn fullPathIsValid(path : [char]) -> bool
{
	pathIsValid(path) && containsAllLetters(path)
}

fn lettersDontRepeat(letters : [const char]) -> bool
{
	for letter in letters
	{
		if vec::count(letters, letter) > 1u
		{
			ret false;
		}
	}
	true
}

fn aIsFirstIfPresent(letters : [char]) -> bool
{
	!vec::contains(letters, 'a') || letters[0] == 'a'
}

fn zIsLastIfPresent(letters : [char]) -> bool
{
	!vec::contains(letters, 'z') || letters[vec::len(letters) - 1u] == 'z'
}

fn containsAllLetters(letters : [char]) -> bool
{
	let allLetters = str::chars("abcdefghijklmnopqrstuvwxyz");
	for letter in allLetters
	{
		if !vec::contains(letters, letter)
		{
			ret false;
		}
	}
	true
}

fn lettersRhyme(first : char, second : char) -> bool
{
	let rhymeSets = [set("ahjk"),set("iy"),set("bcdegptvz"),set("flmnxs"),set("quw"),set("o"),set("r")];
	for rhymeSet in rhymeSets
	{
		if vec::contains(rhymeSet, first) && vec::contains(rhymeSet, second)
		{
			ret true;
		}
	}
	false
}
fn set(input : str) -> [char]
{
	str::chars(input)
}