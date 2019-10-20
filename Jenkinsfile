pipeline {
    agent none
    options {
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test --release'
            }
        }
	}
}

