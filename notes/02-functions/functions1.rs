fn main() {
    let faculty_info  = "Paul Guse, BSc, MSc, BEd (he/him)".to_string();
    let job_title = "Faculty - Mathematics and Computer Science".to_string();
    signature(faculty_info, job_title);
}

fn signature(info: String, title: String) {
    println!("{}\n{}", info, title);
    println!("Albert College - Belleville, Ontario");
}